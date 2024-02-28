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
            <p>{"List of everyone's transactions goes here"}</p>
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
                            .unwrap()
                            .iter()
                            .map(|transaction| {
                                view! {
                                    <tr>
                                        <td>{transaction.id.to_string()}</td>
                                        <td>{transaction.time_processed.to_string()}</td>
                                        <td>{transaction.fromid.to_string()}</td>
                                        <td>{transaction.toid.to_string()}</td>
                                        <td>{transaction.item.clone()}</td>
                                        <td>{transaction.quantity}</td>
                                        <td>{transaction.price}</td>
                                        <td>{transaction.quantity * transaction.price / 10}</td>
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
