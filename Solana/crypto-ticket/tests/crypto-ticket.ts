import type { CryptoTicket } from "../target/types/crypto_ticket";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';

describe("crypto-ticket", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    
    const program = anchor.workspace.CryptoTicket as Program<CryptoTicket>;
    
    // Проверяем, что используем правильный ID программы
    // before(() => {
    //     // console.log("Program ID:", program.programId.toString());
    //     expect(program.programId.toString()).to.equal("8sKVvV5NTamS36qakrS7qm45W2xxgmXPMrmGn4NH2gsm");
    // });
    
    // Тестовые данные
    const ticketId = new anchor.BN(1);
    const price = new anchor.BN(1000000); // 0.001 SOL в lamports
    
    // Генерируем PDA для аккаунта билета
    function findTicketAddress(ticketId: anchor.BN): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from("ticket"),
                ticketId.toArrayLike(Buffer, "le", 8)
            ],
            program.programId
        );
    }
    
    // Генерируем PDA для аккаунта джекпота
    function findJackpotAddress(ticketId: anchor.BN): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from("jackpot"),
                ticketId.toArrayLike(Buffer, "le", 8)
            ],
            program.programId
        );
    }
    
    // Генерируем PDA для первого чанка участников
    function findFirstChunkAddress(ticketId: anchor.BN): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from("participants"),
                ticketId.toArrayLike(Buffer, "le", 8),
                new anchor.BN(0).toArrayLike(Buffer, "le", 8)
            ],
            program.programId
        );
    }
    
  
    it("Is initialized!", async () => {
        // Add your test here.
        const tx = await program.methods.initialize().rpc();
        console.log("Your transaction signature", tx);
    });
    
    // Вспомогательная функция для просмотра IDL
    it("Выводит IDL для проверки названий аккаунтов", async () => {
        // console.log(JSON.stringify(program.idl, null, 2));
    });
    
    it("Инициализирует новый билет", async () => {
        const [ticketAddress] = findTicketAddress(ticketId);
        const [jackpotAddress] = findJackpotAddress(ticketId);
        const [firstChunkAddress] = findFirstChunkAddress(ticketId);
        
        try {
            const tx = await program.methods
                .initTicket(ticketId, price)
                .accounts({
                    ticketAccount: ticketAddress,
                    ticketJackpot: jackpotAddress,
                    firstParticipantsChunk: firstChunkAddress,
                    admin: provider.wallet.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc();
            
            console.log("Транзакция выполнена успешно:", tx);
            
            // Получаем данные созданного билета
            const ticketAccount = await program.account.ticketAccount.fetch(ticketAddress);
            
            // Проверяем корректность данных билета
            expect(ticketAccount.isActive).to.be.true;
            expect(ticketAccount.admin.toString()).to.equal(provider.wallet.publicKey.toString());
            expect(ticketAccount.ticketId.toString()).to.equal(ticketId.toString());
            expect(ticketAccount.price.toString()).to.equal(price.toString());
            expect(ticketAccount.totalParticipants.toString()).to.equal("0");
            
            // Получаем данные джекпота
            const jackpotAccount = await program.account.ticketJackpot.fetch(jackpotAddress);
            
            // Проверяем корректность данных джекпота
            expect(jackpotAccount.totalAmount.toString()).to.equal("0");
            expect(jackpotAccount.isClaimed).to.be.false;
            expect(jackpotAccount.ticketId.toString()).to.equal(ticketId.toString());
            
            // Получаем данные первого чанка
            const firstChunk = await program.account.participantsChunk.fetch(firstChunkAddress);
            
            // Проверяем корректность данных чанка
            expect(firstChunk.ticketId.toString()).to.equal(ticketId.toString());
            expect(firstChunk.chunkIndex.toString()).to.equal("0");
            expect(firstChunk.currentCount.toString()).to.equal("0");
            expect(firstChunk.participants).to.be.empty;
            
        } catch (error) {
            console.error("Ошибка при выполнении теста:", error);
            throw error;
        }
    });
    
    it("Не позволяет повторно инициализировать билет с тем же ID", async () => {
        const [ticketAddress] = findTicketAddress(ticketId);
        const [jackpotAddress] = findJackpotAddress(ticketId);
        const [firstChunkAddress] = findFirstChunkAddress(ticketId);
        
        try {
            await program.methods
                .initTicket(ticketId, price)
                .accounts({
                    ticketAccount: ticketAddress,
                    ticketJackpot: jackpotAddress,
                    firstParticipantsChunk: firstChunkAddress,
                    admin: provider.wallet.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc();
            
            throw new Error("Ожидалась ошибка при повторной инициализации");
        } catch (error: any) {
            // Выводим полное сообщение об ошибке для отладки
            console.log("Полученная ошибка:", error);
            
            if (error.logs) {
                console.log("Логи транзакции:", error.logs);
            }
            
            // Проверяем, что это ошибка симуляции транзакции
            expect(error.toString()).to.include("already in use");
            // Проверяем логи транзакции, если они есть
            if (error.logs) {
                console.log("Логи транзакции:", error.logs);
            }
        }
    });
    
    
});
