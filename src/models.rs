use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::schema::*;
use bcrypt::verify;

// Queryable
// 目的: データベースのクエリ結果を Rust の構造体にマッピングします。
// 機能: load や first などのメソッドを使用してデータベースからデータを取得する際に、結果を自動的にこの構造体に変換します。
// 重要性: データベースとRustコード間のデータ変換を自動化し、型安全性を確保します。

// Selectable
// 目的: 構造体のフィールドをデータベースクエリの SELECT 句に自動的にマッピングします。
// 機能: select(AdminUser::as_select()) のような構文を可能にし、すべてのフィールドを明示的に指定することなく選択できるようにします。
// 重要性: クエリの記述を簡略化し、フィールドの追加や削除時のコードメンテナンスを容易にします。

// Identifiable
// 目的: 構造体が一意の識別子（通常は主キー）を持つことを示します。
// 機能: Diesel の特定の関数やメソッド（例：find）で使用され、レコードの一意性を保証します。
// 重要性: データベース操作（更新、削除など）において、特定のレコードを正確に識別するのに役立ちます。

// Debug
// 目的: 構造体のデバッグ表示を自動的に実装します。
// 機能: println!("{:?}", admin_user) のような形式でオブジェクトを出力できるようにします。
// 重要性: デバッグやログ出力を容易にし、開発プロセスを支援します。

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = admin_users)]
pub struct AdminUser {
    pub id: i64,
    pub email: Option<String>,
    pub password_digest: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl AdminUser {
    pub fn verify_password(&self, password: &str) -> bool {
        match &self.password_digest {
            Some(digest) => verify(password, digest).unwrap_or(false),
            None => false,
        }
    }
}