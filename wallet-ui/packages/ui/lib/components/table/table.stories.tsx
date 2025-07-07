import type {Meta, StoryObj} from '@storybook/react-vite'

import {Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow} from './table'

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories#default-export
const meta = {
    title: 'Chub/Table',
    component: Table,
    parameters: {
        // Optional parameter to center the component in the Canvas. More info: https://storybook.js.org/docs/configure/story-layout
        // layout: 'fullscreen'
    },
    // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/writing-docs/autodocs
    tags: ['autodocs'],
    // More on argTypes: https://storybook.js.org/docs/api/argtypes
    argTypes: {
        // backgroundColor: { control: 'color' },
    },
    render: () => (
        <Table>
            <TableCaption>A list of your recent invoices.</TableCaption>
            <TableHeader>
                <TableRow>
                    <TableHead className='w-[100px]'>Invoice</TableHead>
                    <TableHead>Status</TableHead>
                    <TableHead>Method</TableHead>
                    <TableHead className='text-right'>Amount</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow>
                    <TableCell className='font-medium'>INV001</TableCell>
                    <TableCell>Paid</TableCell>
                    <TableCell>Credit Card</TableCell>
                    <TableCell className='text-right'>$250.00</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell className='font-medium'>Re003</TableCell>
                    <TableCell>Paid</TableCell>
                    <TableCell>Cash</TableCell>
                    <TableCell className='text-right'>$400.00</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    )
} satisfies Meta<typeof Table>

export default meta
type Story = StoryObj<typeof meta>

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const Primary: Story = {}
