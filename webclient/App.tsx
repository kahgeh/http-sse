import React, { useEffect, useState, useCallback } from "react";
import { render } from "react-dom";

function App() {
    const [messages, setMessages] = useState<string[]>([]);

    useEffect(() => {
        var source = new EventSource('http://localhost:8080/event');
        const notify = (e: MessageEvent) => setMessages(prevMessages => [e.data, ...prevMessages]);
        source.addEventListener('message', notify);
        return () => source.removeEventListener('message', notify);
    }, [])
    return <div><ul>{messages.map((msg, i) => <li key={i}>{msg}</li>)}</ul></div>;
}
render(<App />, document.getElementById('app'));