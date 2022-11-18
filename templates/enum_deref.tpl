impl core::ops::Deref for {{ name }} {
    type Target = super::types::Country;

    fn deref(&self) -> &Self::Target {
        match self {
            {% for variant in variants %}{{ name }}::{{ variant }} => super::consts::{{ variant }},
            {% endfor %}
        }
    }
}