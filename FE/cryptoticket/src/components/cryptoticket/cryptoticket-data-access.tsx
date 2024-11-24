'use client'

import {getCryptoticketProgram, getCryptoticketProgramId} from '@project/anchor'
import {useConnection} from '@solana/wallet-adapter-react'
import {Cluster, Keypair, PublicKey} from '@solana/web3.js'
import {useMutation, useQuery} from '@tanstack/react-query'
import {useMemo} from 'react'
import toast from 'react-hot-toast'
import {useCluster} from '../cluster/cluster-data-access'
import {useAnchorProvider} from '../solana/solana-provider'
import {useTransactionToast} from '../ui/ui-layout'

export function useCryptoticketProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getCryptoticketProgramId(cluster.network as Cluster), [cluster])
  const program = getCryptoticketProgram(provider)

  const accounts = useQuery({
    queryKey: ['cryptoticket', 'all', { cluster }],
    queryFn: () => program.account.cryptoticket.all(),
  })

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const initialize = useMutation({
    mutationKey: ['cryptoticket', 'initialize', { cluster }],
    mutationFn: (keypair: Keypair) =>
      program.methods.initialize().accounts({ cryptoticket: keypair.publicKey }).signers([keypair]).rpc(),
    onSuccess: (signature) => {
      transactionToast(signature)
      return accounts.refetch()
    },
    onError: () => toast.error('Failed to initialize account'),
  })

  return {
    program,
    programId,
    accounts,
    getProgramAccount,
    initialize,
  }
}

export function useCryptoticketProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const { program, accounts } = useCryptoticketProgram()

  const accountQuery = useQuery({
    queryKey: ['cryptoticket', 'fetch', { cluster, account }],
    queryFn: () => program.account.cryptoticket.fetch(account),
  })

  const closeMutation = useMutation({
    mutationKey: ['cryptoticket', 'close', { cluster, account }],
    mutationFn: () => program.methods.close().accounts({ cryptoticket: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accounts.refetch()
    },
  })

  const decrementMutation = useMutation({
    mutationKey: ['cryptoticket', 'decrement', { cluster, account }],
    mutationFn: () => program.methods.decrement().accounts({ cryptoticket: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const incrementMutation = useMutation({
    mutationKey: ['cryptoticket', 'increment', { cluster, account }],
    mutationFn: () => program.methods.increment().accounts({ cryptoticket: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const setMutation = useMutation({
    mutationKey: ['cryptoticket', 'set', { cluster, account }],
    mutationFn: (value: number) => program.methods.set(value).accounts({ cryptoticket: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  return {
    accountQuery,
    closeMutation,
    decrementMutation,
    incrementMutation,
    setMutation,
  }
}
