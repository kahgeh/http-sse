import React, { useEffect, useState, useCallback } from "react";
import { render } from "react-dom";
const sseClientId = Date.now();
function App() {
    const [messages, setMessages] = useState<string[]>([]);

    useEffect(() => {
        var source = new EventSource(`http://127.0.0.1:8080/clients/${sseClientId}/events`);
        const notify = (e: MessageEvent) => setMessages(prevMessages => [e.data, ...prevMessages]);
        source.addEventListener('message', notify);
        return () => source.removeEventListener('message', notify);
    }, [])
    return (<div>
        ClientID:{sseClientId}
        <ul>{messages.map((msg, i) => <li key={i}>{msg}</li>)}</ul>
    </div>);
}
render(<App />, document.getElementById('app'));