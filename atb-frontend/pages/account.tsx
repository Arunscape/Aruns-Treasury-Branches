import type { NextPage } from "next";
import { createStyles, Paper, Text, ThemeIcon } from '@mantine/core';
import { ColorSwatch } from 'tabler-icons-react';

import dynamic from 'next/dynamic'
import Passwordless from 'supertokens-auth-react/recipe/passwordless'
import ProtectedPage from "./protectedPage";

const PasswordlessAuthNoSSR = dynamic(
    new Promise((res) =>
        res(Passwordless.PasswordlessAuth)
    ) as any,
    { ssr: false }
)

const useStyles = createStyles((theme) => ({
    card: {
        position: 'relative',
        cursor: 'pointer',
        overflow: 'hidden',
        transition: 'transform 150ms ease, box-shadow 100ms ease',
        padding: theme.spacing.xl,
        paddingLeft: theme.spacing.xl * 2,

        '&:hover': {
            boxShadow: theme.shadows.md,
            transform: 'scale(1.02)',
        },

        '&::before': {
            content: '""',
            position: 'absolute',
            top: 0,
            bottom: 0,
            left: 0,
            width: 6,
            backgroundImage: theme.fn.linearGradient(0, theme.colors.pink[6], theme.colors.orange[6]),
        },
    },
}));

interface CardGradientProps {
    title: string;
    description: string;
}

export function CardGradient({ title, description }: CardGradientProps) {
    const { classes } = useStyles();
    return (
        <Paper withBorder radius="md" className={classes.card}>
            <ThemeIcon
                size="xl"
                radius="md"
                variant="gradient"
                gradient={{ deg: 0, from: 'pink', to: 'orange' }}
            >
                <ColorSwatch size={28} />
            </ThemeIcon>
            <Text size="xl" weight={500} mt="md">
                {title}
            </Text>
            <Text size="sm" mt="sm" color="dimmed">
                {description}
            </Text>
        </Paper>
    );
}

const Account: NextPage = () => <>

    <PasswordlessAuthNoSSR>
        <CardGradient title="Balance" description="0 diamonds" />
    </PasswordlessAuthNoSSR>

</>

export default Account;