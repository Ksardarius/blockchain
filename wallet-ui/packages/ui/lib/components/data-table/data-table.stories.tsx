import type {Meta, StoryObj} from '@storybook/react-vite'

import {DataTable} from './data-table'

// This type is used to define the shape of our data.
// You can use a Zod schema here if you want.
interface Payment {
    id: string
    amount: number
    status: 'pending' | 'processing' | 'success' | 'failed'
    email: string
}

const data: Payment[] = [
    {
        id: '728ed52f',
        amount: 100,
        status: 'pending',
        email: 'm@example.com'
    }
]

const meta = {
    title: 'Chub/DataTable',
    tags: ['autodocs'],
    render: props => <DataTable {...props} />,
    args: {
        data: data
    }
} satisfies Meta<typeof DataTable<Payment>>

export default meta
type Story = StoryObj<typeof meta>

export const Primary: Story = {
    args: {
        columns: [
            {
                accessorKey: 'status',
                header: 'Status'
            },
            {
                accessorKey: 'email',
                header: 'Email'
            },
            {
                accessorKey: 'amount',
                header: 'Amount'
            }
        ]
    }
}

export const FormatedCells: Story = {
    args: {
        columns: [
            {
                accessorKey: 'status',
                header: 'Status'
            },
            {
                accessorKey: 'email',
                header: 'Email'
            },
            {
                accessorKey: 'amount',
                header: () => <div className='text-right'>Amount</div>,
                cell: ({row}) => {
                    const amount = parseFloat(row.getValue('amount'))
                    const formatted = new Intl.NumberFormat('en-US', {
                        style: 'currency',
                        currency: 'USD'
                    }).format(amount)

                    return <div className='text-right font-medium'>{formatted}</div>
                }
            }
        ]
    }
}
