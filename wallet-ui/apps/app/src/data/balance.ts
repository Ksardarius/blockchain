import { atom } from "jotai";
import { get_utxos } from '@bc/wallet'
import { selectedAddressAtom } from "./addresses";
import { loadable } from "jotai/utils";

const refrestTriggerAtom = atom(0)

export interface UTXO {
    prev_tx_id: [],
    prev_out_idx: number,
    value: number
}

export const getBalanceAtom = atom<Promise<UTXO[]>>(async (get) => {
    get(refrestTriggerAtom)
    const address = get(selectedAddressAtom)
    if (address) {
        const utxos = await get_utxos(address);
        return utxos.Ok;
    } else {
        return []
    }
})

export const balanceLoadableAtom = loadable(getBalanceAtom) 
export const refreshABalanceAtom = atom(null, (get, set) => {
    set(refrestTriggerAtom, get(refrestTriggerAtom) + 1)
})