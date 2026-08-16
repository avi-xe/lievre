import { Link, useLocation } from "react-router-dom";
import { cn } from "@/lib/utils";
import { Home, Activity, Bell, User } from "lucide-react";

const navItems = [
  { to: "/", icon: Home, label: "Feed" },
  { to: "/activities", icon: Activity, label: "Activities" },
  { to: "/notifications", icon: Bell, label: "Notifications" },
  { to: "/profile", icon: User, label: "Profile" },
];

export function BottomNav() {
  const location = useLocation();

  return (
    <nav
      className="fixed bottom-0 left-0 right-0 z-50 border-t bg-background lg:hidden"
      role="navigation"
      aria-label="Mobile navigation"
    >
      <div className="flex h-16 items-center justify-around">
        {navItems.map((item) => {
          const Icon = item.icon;
          const isActive = location.pathname === item.to;

          return (
            <Link
              key={item.to}
              to={item.to}
              className={cn(
                "flex flex-col items-center justify-center space-y-1 px-4 py-2 text-xs font-medium transition-colors",
                "rounded-md",
                "hover:bg-muted hover:text-foreground",
                "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
                isActive && "text-primary",
                !isActive && "text-muted-foreground"
              )}
              aria-current={isActive ? "page" : undefined}
              aria-label={item.label}
            >
              <Icon
                className={cn(
                  "h-5 w-5 transition-transform",
                  isActive && "scale-110"
                )}
                aria-hidden="true"
              />
              <span className="text-[10px]">{item.label}</span>
            </Link>
          );
        })}
      </div>
    </nav>
  );
}
