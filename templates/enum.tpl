
/// [ISO 3166-1 alpha-{{version}}] country code in enum.
/// 
/// [ISO 3166-1 alpha-{{version}}]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-{{version}}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature="serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(any(feature="async-graphql", feature="alloc"), derive(::async_graphql::Enum))]
#[repr(u8)]
pub enum {{name}} {
    {% for variant in variants %}
    /// {{ variant.doc }}
    {{ variant.name }},
    {% endfor %}
}

impl core::ops::Deref for {{ name }} {
    type Target = super::types::Country;

    fn deref(&self) -> &Self::Target {
        match self {
            {% for variant in variants %}{{ name }}::{{ variant.name }} => super::consts::{{ variant.name }},
            {% endfor %}
        }
    }
}