import { useState, useEffect, useRef, useCallback } from "react";
import "./ChatInput.css";

type ChatInputProps = {
  onSend: (message: string) => void;
  onChatModeChange: (active: boolean) => void;
  maxLength?: number;
};

export default function ChatInput({
  onSend,
  onChatModeChange,
  maxLength = 1024,
}: ChatInputProps) {
  const [isChatMode, setIsChatMode] = useState(false);
  const [text, setText] = useState("");
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const enterChatMode = useCallback(() => {
    setIsChatMode(true);
    onChatModeChange(true);
  }, [onChatModeChange]);

  const exitChatMode = useCallback(() => {
    setIsChatMode(false);
    setText("");
    onChatModeChange(false);
  }, [onChatModeChange]);

  const sendMessage = useCallback(() => {
    const trimmed = text.trim();
    if (trimmed) {
      onSend(trimmed);
    }
    exitChatMode();
  }, [text, onSend, exitChatMode]);

  // Global Enter listener when chat mode is OFF
  useEffect(() => {
    if (isChatMode) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Enter" && !e.shiftKey && !e.altKey && !e.ctrlKey && !e.metaKey) {
        e.preventDefault();
        enterChatMode();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isChatMode, enterChatMode]);

  // Auto-focus textarea when chat mode activates
  useEffect(() => {
    if (isChatMode) {
      textareaRef.current?.focus();
    }
  }, [isChatMode]);

  const handleTextareaKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Escape") {
      exitChatMode();
      return;
    }
    if (e.key === "Enter" && !e.shiftKey && !e.altKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  return (
    <div className="chat-input-container">
      {isChatMode ? (
        <textarea
          ref={textareaRef}
          className="chat-input-textarea"
          value={text}
          onChange={(e) => setText(e.target.value)}
          onKeyDown={handleTextareaKeyDown}
          maxLength={maxLength}
          rows={1}
          placeholder="Type a message…"
        />
      ) : (
        <div className="chat-input-hint">Press Enter to chat</div>
      )}
    </div>
  );
}
