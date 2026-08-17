import { useEffect, useState } from "react";
import { X, Heart, MessageCircle, UserPlus } from "lucide-react";
import { cn } from "@/lib/utils";

interface ToastProps {
  id: string;
  type: "follow" | "like" | "comment" | "repost";
  actor: string;
  content: string;
  onDismiss: (id: string) => void;
  onClick?: () => void;
}

const icons = {
  follow: UserPlus,
  like: Heart,
  comment: MessageCircle,
  repost: MessageCircle,
};

const colors = {
  follow: "bg-emerald-500",
  like: "bg-rose-500",
  comment: "bg-blue-500",
  repost: "bg-violet-500",
};

export function Toast({ id, type, actor, content, onDismiss, onClick }: ToastProps) {
  const [isVisible, setIsVisible] = useState(false);
  const [isLeaving, setIsLeaving] = useState(false);

  useEffect(() => {
    // Fade in
    requestAnimationFrame(() => setIsVisible(true));

    // Auto-dismiss after 5 seconds
    const timer = setTimeout(() => {
      setIsLeaving(true);
      setTimeout(() => onDismiss(id), 300);
    }, 5000);

    return () => clearTimeout(timer);
  }, [id, onDismiss]);

  const Icon = icons[type];

  return (
    <div
      className={cn(
        "fixed bottom-24 right-4 z-50 flex items-start gap-3 rounded-lg border bg-background p-4 shadow-lg transition-all duration-300",
        isVisible && !isLeaving ? "translate-x-0 opacity-100" : "translate-x-full opacity-0"
      )}
      role="alert"
      aria-live="polite"
    >
      <div className={cn("flex h-8 w-8 items-center justify-center rounded-full", colors[type])}>
        <Icon className="h-4 w-4 text-white" aria-hidden="true" />
      </div>
      <div className="flex-1 min-w-0">
        <p className="text-sm font-medium">{actor}</p>
        <p className="text-sm text-muted-foreground truncate">{content}</p>
      </div>
      <button
        onClick={() => {
          setIsLeaving(true);
          setTimeout(() => onDismiss(id), 300);
        }}
        className="flex h-6 w-6 items-center justify-center rounded-md text-muted-foreground hover:bg-muted"
        aria-label="Dismiss notification"
      >
        <X className="h-4 w-4" />
      </button>
    </div>
  );
}

interface ToastContainerProps {
  toasts: Array<{
    id: string;
    type: "follow" | "like" | "comment" | "repost";
    actor: string;
    content: string;
  }>;
  onDismiss: (id: string) => void;
  onClick?: (id: string) => void;
}

export function ToastContainer({ toasts, onDismiss, onClick }: ToastContainerProps) {
  return (
    <div className="fixed bottom-24 right-4 z-50 flex flex-col gap-2">
      {toasts.map((toast) => (
        <Toast
          key={toast.id}
          {...toast}
          onDismiss={onDismiss}
          onClick={() => onClick?.(toast.id)}
        />
      ))}
    </div>
  );
}
