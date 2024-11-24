import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair} from '@solana/web3.js'
import {Cryptoticket} from '../target/types/cryptoticket'

describe('cryptoticket', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Cryptoticket as Program<Cryptoticket>

  const cryptoticketKeypair = Keypair.generate()

  it('Initialize Cryptoticket', async () => {
    await program.methods
      .initialize()
      .accounts({
        cryptoticket: cryptoticketKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([cryptoticketKeypair])
      .rpc()

    const currentCount = await program.account.cryptoticket.fetch(cryptoticketKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment Cryptoticket', async () => {
    await program.methods.increment().accounts({ cryptoticket: cryptoticketKeypair.publicKey }).rpc()

    const currentCount = await program.account.cryptoticket.fetch(cryptoticketKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment Cryptoticket Again', async () => {
    await program.methods.increment().accounts({ cryptoticket: cryptoticketKeypair.publicKey }).rpc()

    const currentCount = await program.account.cryptoticket.fetch(cryptoticketKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement Cryptoticket', async () => {
    await program.methods.decrement().accounts({ cryptoticket: cryptoticketKeypair.publicKey }).rpc()

    const currentCount = await program.account.cryptoticket.fetch(cryptoticketKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set cryptoticket value', async () => {
    await program.methods.set(42).accounts({ cryptoticket: cryptoticketKeypair.publicKey }).rpc()

    const currentCount = await program.account.cryptoticket.fetch(cryptoticketKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the cryptoticket account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        cryptoticket: cryptoticketKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.cryptoticket.fetchNullable(cryptoticketKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
