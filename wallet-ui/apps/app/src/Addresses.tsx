import React, { useCallback, useEffect } from "react"
import {UserPlus} from 'lucide-react'
// import { addressesSelector } from "./data/addresses";
import { create_wallet } from "@bc/wallet";
import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { addressLoadableAtom, refreshAddressListAtom, selectedAddressAtom } from "./data/addresses";

export const Addresses: React.FC = () => {
    const addresses = useAtomValue(addressLoadableAtom)
    const resetAddresses = useSetAtom(refreshAddressListAtom)
    const [selectedAddress, setSelectedAddress] = useAtom(selectedAddressAtom)

    useEffect(() => {
        if(addresses.state === "hasData") {
            setSelectedAddress(addresses.data[0])
        }
    }, [addresses])

    const registerWallet = useCallback(async () => {
        await create_wallet("123")
        resetAddresses()
    }, [create_wallet, resetAddresses])

    const handleSelect = useCallback(async (event: any) => {
        setSelectedAddress(event.target.value)
    }, [create_wallet, resetAddresses])

    const displayAddress = (address: string) => {
        const start = address.substring(0, 3).toUpperCase()
        const end = address.substring(address.length - 4, address.length - 1).toUpperCase()

        return `0x${start}...${end}`
    }

    if (addresses.state === 'loading') {
        return <>Loading...</>
    }

    if (addresses.state === 'hasError') {
        return <>Error...</>
    }

    return <div className="text-sm text-gray-500">
        User: 
        <select value={selectedAddress || ''} onInput={(event) => handleSelect(event)}>
            {
                addresses.data.map(a => {
                    return <option key={a} value={a}>{displayAddress(a)}</option>
                })
            }
        </select>
        <button onClick={registerWallet}><span className="text-green-600"><UserPlus className="w-4 h-4" /></span></button>
    </div>
}