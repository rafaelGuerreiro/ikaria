import { useEffect, useRef } from 'react';
import './ChatHistory.css';

export type ChatMessage = {
  bubbleId: bigint;
  displayName: string;
  characterLevel: number;
  content: string;
  sentAtMs: number;
};

type ChatHistoryProps = {
  messages: ChatMessage[];
};

function formatTime(ms: number): string {
  const date = new Date(ms);
  return `${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`;
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
          {formatTime(msg.sentAtMs)} {msg.displayName} [{msg.characterLevel}]: {msg.content}
        </div>
      ))}
      <div ref={bottomRef} />
    </div>
  );
}
