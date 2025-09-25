use std::future::Future;

/// 外部サービスの例外をResult型に変換するアダプター
pub fn service_exception_adapter<T, E, F>(service_fn: F) -> impl Fn(T) -> Result<T, E>
where
    F: Fn(T) -> Result<T, E>,
{
    move |input: T| service_fn(input)
}

/// 入力を受け取り、副作用を実行してから入力をそのまま返す
pub fn tee<T, F>(f: F) -> impl Fn(T) -> T
where
    F: Fn(&T),
    T: Clone,
{
    move |input: T| {
        f(&input);
        input
    }
}

/// Vec<Result<T, E>> -> Result<Vec<T>, E>
pub fn sequence<T, E>(results: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    results.into_iter().collect()
}

/// エラー型を変換する（Fromトレイトがない場合に使用）
pub fn map_error<T, E1, E2, F>(result: Result<T, E1>, f: F) -> Result<T, E2>
where
    F: FnOnce(E1) -> E2,
{
    result.map_err(f)
}
