import {useCallback, useState} from 'react'
import {create_transaction, mine_block} from '@bc/wallet'
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '@bc/ui'
import {useAtomValue, useSetAtom} from 'jotai'
import {addressLoadableAtom, selectedAddressAtom} from './data/addresses'
import {balanceLoadableAtom, refreshABalanceAtom, type UTXO} from './data/balance'

// Fee will be static for now. But must be calculated based on tx size.
const FEE_PRICE = 10
const DUST_TRESHOLD = 2
const OPTIMIZED_THESHOLD = 5

const selectUtxoToUse = (utxo: UTXO[], requiredAmount: number): UTXO[] => {
    const requiredTotal = requiredAmount + FEE_PRICE
    const utxoIn: UTXO[] = []

    const draft = utxo.sort((u1, u2) => u2.value - u1.value)
    let currentSum = 0

    for (const utxo of draft) {
        utxoIn.push({
            ...utxo
        })

        currentSum += utxo.value

        if (currentSum >= requiredTotal) {
            // optimize
            if (currentSum === requiredTotal) {
                break
            } else if (
                currentSum - requiredAmount > DUST_TRESHOLD &&
                currentSum - requiredAmount < OPTIMIZED_THESHOLD
            ) {
                break
            } else {
                // change is to high
                // should implement backtracking or branch and bound approac

                // for now will accept any
                break
            }
        }
    }

    if (currentSum < requiredTotal) {
        return []
    } else {
        return utxoIn
    }
}

export const SendBlock: React.FC = () => {
    const ownAddress = useAtomValue(selectedAddressAtom)
    const addresses = useAtomValue(addressLoadableAtom)

    const balance = useAtomValue(balanceLoadableAtom)
    const refreshBalance = useSetAtom(refreshABalanceAtom)

    const [recipientAddress, setRecipienAddress] = useState<string>('')
    const [amount, setAmount] = useState<number | undefined>()
    const createTransaction = useCallback(async () => {
        if (!(balance.state === 'hasData') || !amount || !ownAddress) {
            return
        }

        const utxoIn = selectUtxoToUse(balance.data, amount)

        if (utxoIn.length === 0) {
            throw 'Insufficient Funds'
        }

        await create_transaction(ownAddress, '123', recipientAddress, BigInt(amount), BigInt(FEE_PRICE), utxoIn)
        refreshBalance()
    }, [ownAddress, amount, recipientAddress, balance])

    const mineBlock = useCallback(async () => {
        await mine_block()
        refreshBalance()
    }, [])

    if (balance.state === 'loading') {
        return <>Loading...</>
    }

    if (balance.state === 'hasError') {
        return <>Error...</>
    }

    return (
        <div className='bg-white rounded-lg shadow p-6'>
            <h3 className='font-semibold text-lg mb-4'>Send (Fee: {FEE_PRICE})</h3>
            <form className='space-y-4' onSubmit={createTransaction}>
                <Select onValueChange={v => setRecipienAddress(v)}>
                    <SelectTrigger className='w-full border rounded px-3 py-4'>
                        <SelectValue placeholder={<span className='text-gray-400'>Recipient address</span>} />
                    </SelectTrigger>
                    <SelectContent>
                        {addresses.state === 'hasData' &&
                            addresses.data.map(addr => (
                                <SelectItem key={addr} value={addr}>
                                    {addr}
                                </SelectItem>
                            ))}
                    </SelectContent>
                </Select>
                <input
                    onChange={ev => setAmount(Number(ev.target.value))}
                    type='number'
                    placeholder='Amount (MYC)'
                    className='w-full border rounded px-3 py-2'
                />
                <button
                    type='button'
                    onClick={createTransaction}
                    className='w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700'
                >
                    Send
                </button>
                <button
                    type='button'
                    onClick={mineBlock}
                    className='w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700'
                >
                    Mine
                </button>
            </form>
        </div>
    )
}
