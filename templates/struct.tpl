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