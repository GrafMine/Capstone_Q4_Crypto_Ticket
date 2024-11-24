'use client'

import { useWallet } from '@solana/wallet-adapter-react'
import { WalletButton } from '../solana/solana-provider'
import { AppHero, ellipsify } from '../ui/ui-layout'
import { ExplorerLink } from '../cluster/cluster-ui'
import { useCryptoticketProgram } from './cryptoticket-data-access'
import { CryptoticketCreate, CryptoticketList } from './cryptoticket-ui'

const publicKeys = [
    "C86zQH92UWsbGt7z6s39sabYn8yrXC3QucL12u2sAYah",
];

const CryptoTicketCreate = () => {
    const { programId } = useCryptoticketProgram();
    
    return (
        <AppHero
            title="Crypto-Ticket"
            subtitle={
                `Create a new ticket by clicking the "Create" button. The state of a account is stored on-chain and can be manipulated by calling the program\'s methods.`
            }
        >
            <p className="mb-6">
                <ExplorerLink path={`account/${programId}`} label={ellipsify(programId.toString())} />
            </p>
            <CryptoticketCreate />
        </AppHero>
    )
}

export default function CryptoticketFeature() {
    const { publicKey } = useWallet()
    // const { programId } = useCryptoticketProgram()
    if (publicKey) console.log("publicKey", publicKey.toString());
  return publicKey ? (
    <div>
        {publicKeys.includes(publicKey.toString()) ? <CryptoTicketCreate />: null}
      <CryptoticketList />
    </div>
  ) : (
    <div className="max-w-4xl mx-auto">
      <div className="hero py-[64px]">
        <div className="hero-content text-center">
          <WalletButton />
        </div>
      </div>
    </div>
  )
}
