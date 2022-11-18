extern crate alloc;

use super::StaticMap;
use alloc::borrow::Cow;
use async_graphql::{
    indexmap::IndexMap,
    parser::types::Field,
    registry::{MetaType, MetaTypeId, Registry},
    to_value, ContextSelectionSet, Name, OutputType, Positioned, ServerResult, Value,
};

#[async_graphql::async_trait::async_trait]
impl<
        K: 'static + Send + Sync + Eq + core::hash::Hash + core::fmt::Display,
        V: 'static + Send + Sync + ::serde::Serialize,
    > OutputType for StaticMap<K, V>
{
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("JSONObject")
    }

    fn create_type_info(registry: &mut Registry) -> String {
        registry.create_output_type::<Self, _>(MetaTypeId::Scalar, |_| MetaType::Scalar {
            name: <Self as OutputType>::type_name().to_string(),
            description: Some("A scalar that can represent any JSON Object value."),
            is_valid: |_| true,
            visible: None,
            specified_by_url: None,
            inaccessible: false,
            tags: &[],
        })
    }

    async fn resolve(
        &self,
        _ctx: &ContextSelectionSet<'_>,
        _field: &Positioned<Field>,
    ) -> ServerResult<Value> {
        let mut map = IndexMap::new();
        for (name, value) in self.iter() {
            map.insert(
                Name::new(name.to_string()),
                to_value(value).unwrap_or_default(),
            );
        }
        Ok(Value::Object(map))
    }
}
