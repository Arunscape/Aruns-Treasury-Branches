import express from "express";
import { createServer } from "http";
import { Account, Prisma, PrismaClient, Transaction, User } from '@prisma/client'
import { Server } from "socket.io";

const prisma = new PrismaClient()

const app = express();
const httpServer = createServer(app);

type cuid = string;

async function get_accounts(owner_id: cuid): Promise<Account[]> {
    return await prisma.account.findMany({
        where: {
            owner_id
        }
    })
}

async function create_account(owner_id: cuid): Promise<Account | null> {
    return await prisma.account.create({
        data: {
            owner_id,
            balance: {},
        }
    })
}

// @throws if account has a nonzero balance
async function delete_account(id: cuid): Promise<Account> {

    return await prisma.$transaction(async (prisma) => {
        const { balance } = await prisma.account.findFirst({
            where: {
                id
            }
        });

        for (const [_, value] of Object.entries(balance)) {
            if (value != 0) {
                throw new Error("Nonzero balance")
            }
        }
        return prisma.account.delete({
            where: {
                id
            }
        });

    });
}

// @throws if user already exists
async function create_user(username: string, email: string): Promise<User> {
    return await prisma.user.create({
        data: {
            username,
            email,
        }
    })
}

// @throws if balance is nonzero
async function delete_user(user_id: cuid): Promise<User> {
    return await prisma.user.delete({
        where: {
            id: user_id
        }
    })
}

// @throws if insufficient balance
// todo: actually, the balance json object will also need to keep track of other attributes  (nbt data)
// so can't just add them
async function create_transaction(from_account_id: cuid, to_account_id: cuid, item_id: number, amount: bigint, other_attributes: Prisma.JsonValue): Promise<Transaction> {

    return await prisma.$transaction(async (prisma) => {

        const from: Account = await prisma.account.findFirst({
            where: {
                id: from_account_id

            }
        });

        if (from.balance[item_id] < amount) {
            throw Error("Insufficient balance")
        }

        await prisma.account.update({
            data: {
                balance: {
                    // @ts-ignore
                    ...from.balance,
                    [item_id]: from.balance[item_id] - amount
                }
            },
            where: {
                id: from_account_id
            }
        });

        const to: Account = await prisma.account.findFirst({
            where: {
                id: to_account_id

            }
        });

        await prisma.account.update({
            data: {
                balance: {
                    // @ts-ignore
                    ...to.balance,
                    [item_id]: to.balance[item_id] + amount
                }
            },
            where: {
                id: to_account_id
            }
        });

        return await prisma.transaction.create({
            data: {
                from_account_id,
                to_account_id,
                item_id,
                amount,
                other_attributes
            }
        });
    })


}



app.get('/balance', async (req, res) => {
    const r = await prisma.user.findMany({})

    res.json(r)
})



httpServer.listen(5000);

const io = new Server(3000, {
    // options
    cors: {
        origin: ["http://localhost:8000"]
    }
});

io.on("connection", (socket) => {
    // ...
});

io.listen(8000);