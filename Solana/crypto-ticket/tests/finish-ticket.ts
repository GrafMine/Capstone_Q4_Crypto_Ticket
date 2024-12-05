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

describe("finish-ticket", () => {

    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.CryptoTicket as Program<CryptoTicket>;
    
    // Тестовые данные
    const ticketId = new anchor.BN(9);
    const price = new anchor.BN(1000000); // 0.001 SOL в lamports

    const [ticketAddress] = PublicKey.findProgramAddressSync(
        [
            Buffer.from("ticket"),
            ticketId.toArrayLike(Buffer, "le", 8)
        ],
        program.programId
    );

    const [jackpotAddress] = PublicKey.findProgramAddressSync(
        [
            Buffer.from("jackpot"),
            ticketId.toArrayLike(Buffer, "le", 8)
        ],
        program.programId
    );

    // Тест на проверку завершения продажи билетов
    it("Позволяет корректно закончить продажу билетов", async () => {
        
        await program.methods
            .initTicket(ticketId, price)
            .accounts({
                ticketAccount: ticketAddress,
                ticketJackpot: jackpotAddress,
                admin: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();
    
        const buyer1 = anchor.web3.Keypair.generate();
        const buyer2 = anchor.web3.Keypair.generate();

        await provider.connection.requestAirdrop(buyer1.publicKey, 2 * LAMPORTS_PER_SOL);
        await provider.connection.requestAirdrop(buyer2.publicKey, 2 * LAMPORTS_PER_SOL);
        
        await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(buyer1.publicKey, 2 * LAMPORTS_PER_SOL));
        await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(buyer2.publicKey, 2 * LAMPORTS_PER_SOL));
    
        await program.methods
            .buy(ticketId)
            .accounts({
                ticketAccount: ticketAddress,
                ticketJackpot: jackpotAddress,
                user: buyer1.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([buyer1])
            .rpc();
    
        await program.methods
            .buy(ticketId)
            .accounts({
                ticketAccount: ticketAddress,
                ticketJackpot: jackpotAddress,
                user: buyer2.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([buyer2])
            .rpc();
    
        const ticketAccountBefore = await program.account.ticketAccount.fetch(ticketAddress);
        const jackpotAccountBefore = await program.account.ticketJackpot.fetch(jackpotAddress);
    
        await program.methods
            .finishTicket(ticketId)
            .accounts({
                ticketAccount: ticketAddress,
                ticketJackpot: jackpotAddress,
                user: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();
    
        const ticketAccountAfter = await program.account.ticketAccount.fetch(ticketAddress);
        const jackpotAccountAfter = await program.account.ticketJackpot.fetch(jackpotAddress);
    
        expect(ticketAccountBefore.isActive).to.be.true;
        expect(ticketAccountAfter.isActive).to.be.false;
        expect(ticketAccountAfter.totalParticipants).to.equal(2);
        expect(jackpotAccountAfter.isClaimed).to.be.false;
        expect(jackpotAccountBefore.totalAmount).to.equal(jackpotAccountAfter.totalAmount);
    });
    
    it("Не дозволяє завершити продаж квитків неавторизованому користувачу", async () => {
        const unauthorizedUser = anchor.web3.Keypair.generate();
        
        try {
            await program.methods
                .finishTicket(ticketId)
                .accounts({
                    ticketAccount: ticketAddress,
                    ticketJackpot: jackpotAddress,
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
    
    // Тест на перевірку спроби завершити вже неактивний квиток
    it("Не дозволяє завершити вже неактивний квиток", async () => {
        try {
            await program.methods
                .finishTicket(ticketId)
                .accounts({
                    ticketAccount: ticketAddress,
                    ticketJackpot: jackpotAddress,
                    user: provider.wallet.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc();
            
            assert.fail("Очікувалася помилка TicketNotActive");
        } catch (error) {
            expect(error.toString()).to.include("TicketNotActive");
        }
    });

    it("Не дозволяє завершити продаж квитків без учасників", async () => {
        // Створюємо новий квиток спеціально для цього тесту
        const emptyTicketId = new anchor.BN(2);
        const [emptyTicketAddress] = await PublicKey.findProgramAddressSync(
            [Buffer.from("ticket"), emptyTicketId.toArrayLike(Buffer, "le", 8)],
            program.programId
        );
        const [emptyJackpotAddress] = await PublicKey.findProgramAddressSync(
            [Buffer.from("jackpot"), emptyTicketId.toArrayLike(Buffer, "le", 8)],
            program.programId
        );

        // Ініціалізуємо квиток, але не купуємо жодного
        await program.methods
            .initTicket(emptyTicketId, price)
            .accounts({
                ticketAccount: emptyTicketAddress,
                ticketJackpot: emptyJackpotAddress,
                admin: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        try {
            await program.methods
                .finishTicket(emptyTicketId)
                .accounts({
                    ticketAccount: emptyTicketAddress,
                    ticketJackpot: emptyJackpotAddress,
                    user: provider.wallet.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc();
            
            assert.fail("Очікувалася помилка NoParticipants");
        } catch (error) {
            expect(error.toString()).to.include("NoParticipants");
        }
    });

    it("Перевіряє стан джекпоту після завершення продажу", async () => {
        // Отримуємо стан джекпоту до та після завершення
        const jackpotBefore = await program.account.ticketJackpot.fetch(jackpotAddress);
        
        await program.methods
            .finishTicket(ticketId)
            .accounts({
                ticketAccount: ticketAddress,
                ticketJackpot: jackpotAddress,
                user: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        const jackpotAfter = await program.account.ticketJackpot.fetch(jackpotAddress);

        // Перевіряємо, що кошти залишились недоторканими
        expect(jackpotAfter.totalAmount.toString()).to.equal(jackpotBefore.totalAmount.toString());
        expect(jackpotAfter.isClaimed).to.be.false;
    });

    it("Перевіряє події після завершення продажу квитків", async () => {
        // Створюємо слухача подій
        const eventPromise = new Promise((resolve, _) => {
            program.addEventListener("TicketFinishedEvent", (event, _) => {
                resolve(event);
            });
        });

        // Виконуємо транзакцію
        await program.methods
            .finishTicket(ticketId)
            .accounts({
                ticketAccount: ticketAddress,
                ticketJackpot: jackpotAddress,
                user: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

    });

    it("Перевіряє можливість купити квиток після завершення продажу", async () => {
        // Спочатку завершуємо продаж
        await program.methods
            .finishTicket(ticketId)
            .accounts({
                ticketAccount: ticketAddress,
                ticketJackpot: jackpotAddress,
                user: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        // Створюємо нового покупця
        const lateBuyer = anchor.web3.Keypair.generate();
        await provider.connection.requestAirdrop(lateBuyer.publicKey, 2 * LAMPORTS_PER_SOL);

        try {
            await program.methods
                .buy(ticketId)
                .accounts({
                    ticketAccount: ticketAddress,
                    ticketJackpot: jackpotAddress,
                    user: lateBuyer.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([lateBuyer])
                .rpc();
            
            assert.fail("Очікувалася помилка TicketNotActive");
        } catch (error) {
            expect(error.toString()).to.include("TicketNotActive");
        }
    });
});