import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "@/contexts/useAuth";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Separator } from "@/components/ui/separator";
import { Bell, Menu, Settings, LogOut, User, Map } from "lucide-react";

interface HeaderProps {
  onMenuToggle?: () => void;
  notificationCount?: number;
}

export function Header({ onMenuToggle, notificationCount = 0 }: HeaderProps) {
  const { user, logout } = useAuth();
  const navigate = useNavigate();

  const handleLogout = () => {
    logout();
    navigate("/login");
  };

  return (
    <header
      className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
      role="banner"
    >
      <div className="container mx-auto flex h-14 items-center px-4">
        {/* Mobile menu button */}
        <Button
          variant="ghost"
          size="icon"
          className="mr-2 lg:hidden"
          onClick={onMenuToggle}
          aria-label="Toggle navigation menu"
        >
          <Menu className="h-5 w-5" />
        </Button>

        {/* Logo */}
        <Link
          to="/"
          className="mr-6 flex items-center space-x-2 transition-opacity hover:opacity-80 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 rounded-md"
          aria-label="Lièvre home"
        >
          <Map className="h-6 w-6 text-primary" aria-hidden="true" />
          <span className="font-bold">Lièvre</span>
        </Link>

        <div className="flex-1" />

        {/* Right side actions */}
        <div className="flex items-center space-x-2">
          {/* Notifications */}
          <Button
            variant="ghost"
            size="icon"
            asChild
            className="relative focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          >
            <Link
              to="/notifications"
              aria-label={`Notifications${notificationCount > 0 ? ` (${notificationCount} unread)` : ""}`}
            >
              <Bell className="h-5 w-5" aria-hidden="true" />
              {notificationCount > 0 && (
                <Badge
                  variant="destructive"
                  className="absolute -right-1 -top-1 h-5 min-w-5 rounded-full px-1 text-xs"
                  aria-hidden="true"
                >
                  {notificationCount > 99 ? "99+" : notificationCount}
                </Badge>
              )}
            </Link>
          </Button>

          <Separator orientation="vertical" className="h-6 mx-1 hidden sm:block" />

          {/* Profile menu */}
          <DropdownMenu>
            <DropdownMenuTrigger
              className="rounded-full focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 outline-none"
              aria-label="User menu"
            >
              <Avatar className="h-8 w-8">
                <AvatarImage src="/avatars/user.jpg" alt="" />
                <AvatarFallback>{user?.username?.[0]?.toUpperCase() || "U"}</AvatarFallback>
              </Avatar>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-48">
              <DropdownMenuItem asChild>
                <Link
                  to="/profile"
                  className="flex items-center cursor-pointer focus-visible:bg-accent focus-visible:text-accent-foreground"
                >
                  <User className="mr-2 h-4 w-4" aria-hidden="true" />
                  Profile
                </Link>
              </DropdownMenuItem>
              <DropdownMenuItem asChild>
                <Link
                  to="/settings"
                  className="flex items-center cursor-pointer focus-visible:bg-accent focus-visible:text-accent-foreground"
                >
                  <Settings className="mr-2 h-4 w-4" aria-hidden="true" />
                  Settings
                </Link>
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem
                className="flex items-center cursor-pointer text-destructive focus-visible:bg-destructive/10 focus-visible:text-destructive"
                onSelect={(e) => {
                  e.preventDefault();
                  handleLogout();
                }}
              >
                <LogOut className="mr-2 h-4 w-4" aria-hidden="true" />
                Log out
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>
    </header>
  );
}
