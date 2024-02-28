use {crate::serverfns::*, leptos::*, leptos_router::*};

#[component]
pub fn TransactionsPage() -> impl IntoView {
    //let transactions = create_resource(
    //    || (),
    //    |_| async move { transactions::get_transactions().await },
    //);
    //
    let transaction_action = create_action(|input: &String| {
        let input = input.clone();
        async move { todo!() }
    });

    view! {
        <>
            <h1>Transactions</h1>
            <table class="table">
                <thead>
                    <th>id</th>
                    <th>time</th>
                    <th>from_account</th>
                    <th>to_account</th>
                    <th>item</th>
                    <th>quantity</th>
                    <th>price</th>
                    <th>total cost</th>
                </thead>
                <tbody>

                    <Await future=|| transactions::get_transactions() let:data>
                        // {format!("{:#?}", (*data).clone().unwrap())}

                        {(*data)
                            .clone()
                            .unwrap_or(Vec::new())
                            .iter()
                            .map(|transaction| {
                                view! {
                                    <tr>
                                        <td>{transaction.id}</td>
                                        <td>{transaction.time_processed.to_string()}</td>
                                        <td>{transaction.fromid.to_string()}</td>
                                        <td>{transaction.toid.to_string()}</td>
                                        <td>{transaction.item.clone()}</td>
                                        <td>{transaction.quantity}</td>
                                        <td>{transaction.format_price()}</td>
                                        <td>{transaction.format_cost_basis()}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}

                    </Await>
                </tbody>
            </table>
        </>
    }
}

#[derive(Params)]
struct TransactionsByItemParams {
    item: String,
}

#[component]
pub fn TransactionsByItemPage() -> impl IntoView {
    //let { item }  = use_params::<TransactionsByItemParams>();
    //
    //
    view! {
        <>
        <h1></h1>
        </>
    }
}

// TODO: Black-Scholes / Merton Equation for efficiently pricing options
