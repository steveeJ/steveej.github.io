use pulldown_cmark::{Options, Parser};
use yew::Html;

/// Renders a string of Markdown to HTML
pub fn render_markdown(markdown_input: &str) -> Html {
    // Set up options and parser.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // Use JS to set the rendered markdown to the innerHTML of a div
    let raw_html = stdweb::js! {
        var div = document.createElement("div");
        div.innerHTML = @{html_output};
        console.log(div);
        return div;
    };

    // Parse the div into a node
    let node = {
        use stdweb::unstable::TryFrom;
        stdweb::web::Node::try_from(raw_html).expect("markdown parsed to invalid html")
    };

    // TODO: figure out why this doesn't work
    // let node = stdweb::web::Node::from_html(&html_output).expect("markdown parsed to invalid html");

    // Put the node in a VRef
    yew::virtual_dom::vnode::VNode::VRef(node)
}
