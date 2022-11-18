{{doc}}
{{derives}}
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct {{name}} {
    {% for field in fields %}pub(crate) {{ field.name }}: {{ field.ty }},
    {% endfor %}
}

impl {{name}} {
    {% for field in fields %}
    {{field.doc}}
    #[inline]
    pub const fn {{ field.getter }}(&self) -> {{ field.ty }} {
        self.{{ field.name }}
    }
    {% endfor %}
}

#[cfg(all(feature = "async-graphql", feature = "alloc"))]
mod {{ mod }}_graphql {
    use async_graphql::Object;
    use super::*;

    #[Object]
    impl {{name}} {
        {% for field in fields %}
        {{field.doc}}
        #[graphql(name = "{{ field.getter }}")]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "async-graphql", feature = "alloc"))))]
        #[inline]
        pub async fn graphql_{{ field.getter }}(&self) -> {{ field.ty }} {
            self.{{ field.name }}
        }
        {% endfor %} 
    }
}



