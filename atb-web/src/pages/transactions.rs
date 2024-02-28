use {crate::serverfns::*, leptos::*, leptos_router::*};

#[component]
pub fn TransactionsPage() -> impl IntoView {
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

#[derive(Params, PartialEq, Debug)]
struct TransactionsByItemParams {
    item: String,
}

#[component]
pub fn TransactionsByItemPage() -> impl IntoView {
    let params = use_params::<TransactionsByItemParams>();

    let item = move || params.with(|p| p.as_ref().map(|p| p.item.clone()).unwrap_or_default());

    view! {
        <>
        <h1>{format!("💰 {}", item())}</h1>
        //<Await future=|| transactions::get_transactions_by_item(item().clone()) let:data>
        //    <div>{format!("{:#?}", (*data).clone())}</div>
        //</Await>
        </>
    }
}

// TODO: Black-Scholes / Merton Equation for efficiently pricing options
