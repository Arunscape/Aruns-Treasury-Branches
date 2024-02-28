use {crate::serverfns::*, leptos::*};

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
                                let tot = transaction.quantity * transaction.price;
                                let before_decimal = tot / 100;
                                let after_decimal = tot % 100;
                                view! {
                                    <tr>
                                        <td>{transaction.id}</td>
                                        <td>{transaction.time_processed.to_string()}</td>
                                        <td>{transaction.fromid.to_string()}</td>
                                        <td>{transaction.toid.to_string()}</td>
                                        <td>{transaction.item.clone()}</td>
                                        <td>{transaction.quantity}</td>
                                        <td>{transaction.price}</td>
                                        <td>{format!("💰{before_decimal}.{after_decimal:02}")}</td>
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

// TODO: Black-Scholes / Merton Equation for efficiently pricing options
