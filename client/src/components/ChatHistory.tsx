import { useEffect, useRef } from 'react';
import './ChatHistory.css';

export type ChatMessage = {
  bubbleId: bigint;
  displayName: string;
  content: string;
  sentAtMs: number;
};

type ChatHistoryProps = {
  messages: ChatMessage[];
};

function formatTime(ms: number): string {
  const date = new Date(ms);
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

export default function ChatHistory({ messages }: ChatHistoryProps) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  return (
    <div className="chat-history">
      {messages.map((msg) => (
        <div key={String(msg.bubbleId)} className="chat-history-entry">
          <span className="chat-history-time">{formatTime(msg.sentAtMs)}</span>
          <span className="chat-history-name">{msg.displayName}</span>
          <span className="chat-history-content">{msg.content}</span>
        </div>
      ))}
      <div ref={bottomRef} />
    </div>
  );
}
