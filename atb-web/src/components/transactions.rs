use leptos::*;

#[component]
// TODO: see if we can use a &[Transaction] instead of moving an object in
//pub fn TransactionsTable(transactions: impl Iter<item = atb_types::Transaction>) -> impl IntoView {
pub fn TransactionsTable(transactions: Vec<atb_types::Transaction>) -> impl IntoView {
    view! {
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

                {transactions
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
            </tbody>
        </table>
    }
}
