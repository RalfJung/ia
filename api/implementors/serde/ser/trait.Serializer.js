(function() {var implementors = {};
implementors["erased_serde"] = [{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;'a mut dyn <a class=\"trait\" href=\"erased_serde/trait.Serializer.html\" title=\"trait erased_serde::Serializer\">Serializer</a>",synthetic:false,types:["erased_serde::ser::Serializer"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;'a mut (dyn <a class=\"trait\" href=\"erased_serde/trait.Serializer.html\" title=\"trait erased_serde::Serializer\">Serializer</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>)",synthetic:false,types:["erased_serde::ser::Serializer"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;'a mut (dyn <a class=\"trait\" href=\"erased_serde/trait.Serializer.html\" title=\"trait erased_serde::Serializer\">Serializer</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>)",synthetic:false,types:["erased_serde::ser::Serializer"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;'a mut (dyn <a class=\"trait\" href=\"erased_serde/trait.Serializer.html\" title=\"trait erased_serde::Serializer\">Serializer</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>)",synthetic:false,types:["erased_serde::ser::Serializer"]},];
implementors["serde_cbor"] = [{text:"impl&lt;'a, W&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;'a mut <a class=\"struct\" href=\"serde_cbor/ser/struct.Serializer.html\" title=\"struct serde_cbor::ser::Serializer\">Serializer</a>&lt;W&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,&nbsp;</span>",synthetic:false,types:["serde_cbor::ser::Serializer"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
