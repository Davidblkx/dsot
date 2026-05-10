import type { Meta, StoryObj } from '@storybook/vue3-vite';
import { action } from 'storybook/actions';

interface ButtonArgs {
    value: string;
    onClick: () => void;
    disabled?: boolean;
}

const meta: Meta<ButtonArgs> = {
    title: 'Design System/Components/Button',
    argTypes: {
        value: { control: 'text' },
        disabled: { control: 'boolean' },
    }
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
    render: (args) => ({
        render: () => <button type="button" onClick={args.onClick} disabled={args.disabled}>{args.value}</button>,
    }),
    args: {
        value: 'Button',
        onClick: action('clicked'),
        disabled: false,
    }
};
