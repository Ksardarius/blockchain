import { get_all_wallets } from '@bc/wallet'
import { atom } from 'jotai'
import { loadable } from 'jotai/utils'

const refrestTriggerAtom = atom(0)

const getAddressesAtom = atom(async (get) => {
    get(refrestTriggerAtom)
    return await get_all_wallets()
})
export const addressLoadableAtom = loadable(getAddressesAtom) 
export const refreshAddressListAtom = atom(null, (get, set) => {
    set(refrestTriggerAtom, get(refrestTriggerAtom) + 1)
})

export const selectedAddressAtom = atom<string | null>(null)
