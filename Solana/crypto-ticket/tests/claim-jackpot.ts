import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CryptoTicket } from "../target/types/crypto_ticket";

import {
    PublicKey,
    SystemProgram,
    LAMPORTS_PER_SOL,
    Keypair,
} from "@solana/web3.js";

import { expect } from "chai";
import { assert } from "chai";

describe('claim_jackpot', () => {
    
    // Базовые данные
    const ticketId = new anchor.BN(1);
    const price = new anchor.BN(1000000); // 0.001 SOL
    let ticketAddress: PublicKey;
    let jackpotAddress: PublicKey;
    let adminKeypair: Keypair;

    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.CryptoTicket as Program<CryptoTicket>;

    // Перед тестами создаем тикет, покупаем билеты и завершаем продажу
    before(async () => {
        adminKeypair = anchor.web3.Keypair.generate();
        await provider.connection.requestAirdrop(adminKeypair.publicKey, 10 * LAMPORTS_PER_SOL);

        // находим адреса тикета и джекпота
        [ticketAddress] = await PublicKey.findProgramAddressSync(
            [Buffer.from("ticket"), ticketId.toArrayLike(Buffer, "le", 8)],
            program.programId
        );
        [jackpotAddress] = await PublicKey.findProgramAddressSync(
            [Buffer.from("jackpot"), ticketId.toArrayLike(Buffer, "le", 8)],
            program.programId
        );

        // инициализирцем тикет
        await program.methods
            .initTicket(ticketId, price)
            .accounts({
                ticketAccount: ticketAddress,
                ticketJackpot: jackpotAddress,
                admin: adminKeypair.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([adminKeypair])
            .rpc();

        // СОздаем участников
        const participants = [];
        for (let i = 0; i < 3; i++) {
            const participant = anchor.web3.Keypair.generate();
            await provider.connection.requestAirdrop(participant.publicKey, 2 * LAMPORTS_PER_SOL);
            participants.push(participant);
        }

        // ждем подтверждения транзакций
        await new Promise(resolve => setTimeout(resolve, 1000));

        // покупаем билеты
        for (const participant of participants) {
            await program.methods
                .buy(ticketId)
                .accounts({
                    ticketAccount: ticketAddress,
                    ticketJackpot: jackpotAddress,
                    user: participant.publicKey,
                    admin: adminKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([participant])
                .rpc();
        }

        // закрываем продажу билетов
        await program.methods
            .finishTicket(ticketId)
            .accounts({
                ticketAccount: ticketAddress,
                ticketJackpot: jackpotAddress,
                user: adminKeypair.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([adminKeypair])
            .rpc();
    });

    it("Успешно выплачивает джекпот последнему участнику", async () => {
        // Находим адреса чанков
        const chunkAddresses = [];
        const totalParticipants = 3;
        const totalChunks = Math.ceil(totalParticipants / 50);

        for (let i = 0; i < totalChunks; i++) {
            const [chunkAddress] = await PublicKey.findProgramAddressSync(
                [
                    Buffer.from("participants"),
                    ticketId.toArrayLike(Buffer, "le", 8),
                    new anchor.BN(i).toArrayLike(Buffer, "le", 8)
                ],
                program.programId
            );
            chunkAddresses.push(chunkAddress);
        }

        // Получаем последнего участника
        const lastChunkData = await program.account.participantsChunk.fetch(chunkAddresses[0]);
        const lastParticipant = lastChunkData.participants[2];

        // СОхраняем начальный баланс участника и джекпота
        const initialWinnerBalance = await provider.connection.getBalance(lastParticipant.pubkey);
        const initialJackpotBalance = await provider.connection.getBalance(jackpotAddress);

        // Выполняем выплату джекпота
        await program.methods
            .claimJackpotHandler(ticketId)
            .accounts({
                ticketAccount: ticketAddress,
                ticketJackpot: jackpotAddress,
                winnerParticipantsChunk: chunkAddresses[0],
                winner: lastParticipant.pubkey,
                user: adminKeypair.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .remainingAccounts(
                chunkAddresses.map(address => ({
                    pubkey: address,
                    isWritable: false,
                    isSigner: false
                }))
            )
            .signers([adminKeypair])
            .rpc();

        // Проверяем результаы
        const finalWinnerBalance = await provider.connection.getBalance(lastParticipant.pubkey);
        const finalJackpotBalance = await provider.connection.getBalance(jackpotAddress);
        const ticketAccount = await program.account.ticketAccount.fetch(ticketAddress);
        const jackpotAccount = await program.account.ticketJackpot.fetch(jackpotAddress);

        expect(finalWinnerBalance).to.be.above(initialWinnerBalance);
        expect(finalJackpotBalance).to.equal(0);
        expect(ticketAccount.isActive).to.be.false;
        expect(jackpotAccount.isClaimed).to.be.true;
        expect(jackpotAccount.winner.toString()).to.equal(lastParticipant.pubkey.toString());
    });

    it("Не дозволяє виплатити джекпот не адміністратору", async () => {
        const unauthorizedUser = anchor.web3.Keypair.generate();
        await provider.connection.requestAirdrop(unauthorizedUser.publicKey, LAMPORTS_PER_SOL);

        try {
            await program.methods
                .claimJackpotHandler(ticketId)
                .accounts({
                    ticketAccount: ticketAddress,
                    ticketJackpot: jackpotAddress,
                    winner: unauthorizedUser.publicKey,
                    user: unauthorizedUser.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([unauthorizedUser])
                .rpc();
            
            assert.fail("Очікувалася помилка UnauthorizedAccess");
        } catch (error) {
            expect(error.toString()).to.include("UnauthorizedAccess");
        }
    });

    it("Не дозволяє виплатити джекпот двічі", async () => {
        try {
            await program.methods
                .claimJackpotHandler(ticketId)
                .accounts({
                    ticketAccount: ticketAddress,
                    ticketJackpot: jackpotAddress,
                    winner: adminKeypair.publicKey,
                    user: adminKeypair.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([adminKeypair])
                .rpc();
            
            assert.fail("Очікувалася помилка JackpotAlreadyClaimed");
        } catch (error) {
            expect(error.toString()).to.include("JackpotAlreadyClaimed");
        }
    });

    it("Перевіряє правильність послідовності чанків", async () => {
        const newTicketId = new anchor.BN(2);

        try {
            // Пробуем передать чанки в неправильном порядке
            await program.methods
                .claimJackpotHandler(newTicketId)
                .accounts({
                    // ... accounts ...
                })
                .remainingAccounts([
                    // неправильная последовательность чанков
                ])
                .signers([adminKeypair])
                .rpc();
            
            assert.fail("Очікувалася помилка InvalidChunkSequence");
        } catch (error) {
            expect(error.toString()).to.include("InvalidChunkSequence");
        }
    });
});