use {
    crate::{components::*, serverfns::*},
    leptos::*,
    leptos_router::*,
};

#[component]
pub fn TransactionsPage() -> impl IntoView {
    let transactions = create_resource(|| (), |_| async move { get_transactions().await });
    view! {
        <>
            <h1>Transactions</h1>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| {
                    let err = format!("{:?}", errors.get());
                    log::error!("{err:?}");
                    view! { errors.get }
                }>
                    {move || {
                        transactions
                            .get()
                            .map(|x| x.map(|x| view! { <TransactionsTable transactions=x/> }))
                    }}

                </ErrorBoundary>
            </Suspense>
        </>
    }
}

#[derive(Params, PartialEq, Debug)]
struct TransactionsByItemParams {
    item: Option<String>,
}

#[component]
pub fn TransactionsByItemPage() -> impl IntoView {
    let params = use_params::<TransactionsByItemParams>();

    let item = move || params.with(|p| p.as_ref().map(|p| p.item.clone()).unwrap_or_default());

    let transactions = create_resource(
        || (),
        move |_| async move { get_transactions_by_item(item().unwrap_or_default()).await },
    );

    view! {
        <>
            <h1>{format!("💰 {}", item().unwrap_or_default())}</h1>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| {
                    let err = format!("{:?}", errors.get());
                    log::error!("{err:?}");
                    view! { format!("{:?}", errors.get()) }
                }>
                    {move || {
                        transactions
                            .get()
                            .map(|x| x.map(|x| view! { <TransactionsTable transactions=x/> }))
                    }}

                </ErrorBoundary>
            </Suspense>
        </>
    }
}

// TODO: Black-Scholes / Merton Equation for efficiently pricing options
