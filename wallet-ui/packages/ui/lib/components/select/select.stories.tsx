import type {Meta, StoryObj} from '@storybook/react-vite'

import {Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue} from './select'

const meta: Meta<typeof Select> = {
    title: 'Chub/Select',
    component: Select,
    parameters: {
        // Optional parameter to center the component in the Canvas. More info: https://storybook.js.org/docs/configure/story-layout
    },
    // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/writing-docs/autodocs
    tags: ['autodocs'],
    // More on argTypes: https://storybook.js.org/docs/api/argtypes
    argTypes: {
        // backgroundColor: { control: 'color' },
    },
    // Use `fn` to spy on the onClick arg, which will appear in the actions panel once invoked: https://storybook.js.org/docs/essentials/actions#action-args
    // args: {onClick: fn()},
    render: props => (
        <Select {...props}>
            <SelectTrigger className='w-[180px]'>
                <SelectValue placeholder='Select a fruit' />
            </SelectTrigger>
            <SelectContent>
                <SelectGroup>
                    <SelectLabel>Fruits</SelectLabel>
                    <SelectItem value='apple'>Apple</SelectItem>
                    <SelectItem value='banana'>Banana</SelectItem>
                    <SelectItem value='blueberry'>Blueberry</SelectItem>
                    <SelectItem value='grapes'>Grapes</SelectItem>
                    <SelectItem value='pineapple'>Pineapple</SelectItem>
                </SelectGroup>
            </SelectContent>
        </Select>
    )
} satisfies Meta<typeof Select>

export default meta
type Story = StoryObj<typeof meta>

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const Primary: Story = {
    args: {
    }
}
