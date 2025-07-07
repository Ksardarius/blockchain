import { useAtomValue } from "jotai";
import { type FC } from "react";
import { balanceLoadableAtom } from "./data/balance";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@bc/ui'

function toHexString(byteArray: any) {
    return byteArray.map((byte: any) => byte.toString(16).padStart(2, '0')).join('')
}

export const Balance: FC = () => {
    const balance = useAtomValue(balanceLoadableAtom)

    if (balance.state === 'loading') {
        return <>Loading...</>
    }

    if (balance.state === 'hasError') {
        return <>Error...</>
    }

    const prepareTxId = (txId: any) => {
        const hex = toHexString(txId)
        const start = hex.substring(0, 3).toUpperCase()
        const end = hex.substring(hex.length - 4, hex.length - 1).toUpperCase()

        return `0x${start}...${end}`
    }

    return <Table>
        <TableHeader>
            <TableRow>
                <TableHead className='w-[100px]'>Transaction</TableHead>
                <TableHead>Number</TableHead>
                <TableHead>Amount</TableHead>
            </TableRow>
        </TableHeader>
        <TableBody>
            {
                balance.data.map((b, i) => <TableRow key={i}>
                    <TableCell className='font-medium'>{prepareTxId(b.prev_tx_id)}</TableCell>
                    <TableCell className='font-medium'>{b.prev_out_idx}</TableCell>
                    <TableCell><span className="text-2xl font-bold text-green-600">{b.value}</span></TableCell>
                </TableRow>)
            }
        </TableBody>
    </Table>
}