# Peloton Design System

## Foundation: shadcn/ui + Tailwind CSS

shadcn/ui provides copy-paste React components built on Radix UI primitives, styled with Tailwind CSS. We customize via CSS variables (HSL) in `globals.css`.

### Color Tokens (mapped to our palette)

```css
--primary: 358 75% 59%;        /* #E63946 — Primary red */
--primary-foreground: 0 0% 100%;
--secondary: 213 52% 23%;      /* #1D3557 — Dark blue */
--secondary-foreground: 0 0% 100%;
--accent: 199 58% 53%;         /* #457B9D — Ride blue */
--accent-foreground: 0 0% 100%;
--muted: 220 14% 80%;          /* #A8B4BF — Gray */
--muted-foreground: 220 9% 46%;
--background: 210 20% 98%;     /* #F5F7F9 — Light bg */
--foreground: 213 52% 23%;     /* #1D3557 — Dark text */
--destructive: 358 75% 59%;
--border: 214 20% 90%;
--ring: 358 75% 59%;
--card: 0 0% 100%;
--card-foreground: 213 52% 23%;
```

### Sport Colors (custom CSS variables)

```css
--ride: 204 48% 44%;           /* #457B9D */
--run: 168 42% 42%;            /* #2A9D8F */
--swim: 199 43% 24%;           /* #264653 */
--walk: 43 70% 60%;            /* #E9C46A */
--hike: 28 65% 60%;            /* #F4A261 */
```

---

## Component Mapping by Screen

### 1. Authentication (`/login`, `/register`)

| Screen | shadcn/ui Components | Notes |
|--------|---------------------|-------|
| Login | `Card`, `Input`, `Button`, `Label`, `Separator` | Email + password form |
| Register | `Card`, `Input`, `Button`, `Label`, `Separator` | Name + email + password + confirm |
| Fediverse Login | `Card`, `Input`, `Button`, `Dialog` | WebFinger lookup in dialog |
| Forgot Password | `Card`, `Input`, `Button` | Simple email input |

**Layout**: Centered `Card` with `max-w-md`, consistent padding.

### 2. Feed / Home (`/`)

| Screen | shadcn/ui Components | Notes |
|--------|---------------------|-------|
| Activity Card | `Card`, `Badge`, `Avatar`, `Button`, `Separator` | Main feed item |
| Quick Stats Bar | `Card` + custom stat layout | Distance, time, elevation |
| Create Post | `Dialog` or `Sheet` | Activity share composer |
| Empty State | `Empty` | No activities yet |

**Layout**: Single column, `max-w-2xl`, responsive padding.

### 3. Activity Detail (`/activity/:id`)

| Screen | shadcn/ui Components | Notes |
|--------|---------------------|-------|
| Header | `Card`, `Badge`, `Avatar` | Title, type badge, user |
| Map View | Custom (Leaflet/Mapbox) | Route visualization |
| Stats Grid | `Card` + grid layout | Distance, time, speed, elevation |
| Charts | `Chart` (Recharts) | Speed/elevation over time |
| Splits Table | `Table` or `DataTable` | Per-km splits |
| Comments | `Card`, `Input`, `Button` | Comment thread |
| Engagement | `Button` (ghost), `Badge` | Like, comment, repost |

**Layout**: Two-column on desktop (map left, stats right), stacked on mobile.

### 4. Explore / Search (`/explore`)

| Screen | shadcn/ui Components | Notes |
|--------|---------------------|-------|
| Search Bar | `Command` or `Input` + `Popover` | Global search |
| Filter Chips | `Toggle Group`, `Badge` | Sport type, distance, date |
| Activity Grid | `Card` grid | Activity thumbnails |
| User Cards | `Card`, `Avatar`, `Button` | User discovery |
| Map View | Custom | Activity heatmap |

**Layout**: Filter bar top, grid below, optional map toggle.

### 5. Profile (`/profile/:id`)

| Screen | shadcn/ui Components | Notes |
|--------|---------------------|-------|
| Profile Header | `Card`, `Avatar`, `Button`, `Badge` | Name, stats, follow button |
| Activity Tabs | `Tabs` | Activities, Stats, Kudos |
| Stats Overview | `Card` grid | Total distance, activities, etc. |
| Activity List | `Card`, `Badge`, `Table` | Recent activities |
| Settings | `Card`, `Input`, `Switch`, `Select` | Profile settings |

**Layout**: Header card, tabbed content below.

### 6. Record Activity (`/record`)

| Screen | shadcn/ui Components | Notes |
|--------|---------------------|-------|
| Activity Type | `Toggle Group`, `Badge` | Ride, Run, Swim, Walk, Hike |
| Manual Entry | `Card`, `Input`, `Select`, `Button` | Distance, time, date |
| GPX Upload | `Card`, `Input` (file), `Progress` | File upload with progress |
| GPS Recording | `Card`, `Button`, `Badge` | Live recording controls |

**Layout**: Step-by-step wizard or single card form.

### 7. Notifications (`/notifications`)

| Screen | shadcn/ui Components | Notes |
|--------|---------------------|-------|
| Notification List | `Card`, `Avatar`, `Badge`, `Separator` | Activity notifications |
| Notification Bell | `Button` (ghost), `Badge` | Header icon with count |
| Mark All Read | `Button` (ghost) | Bulk action |

**Layout**: Single column list, `max-w-xl`.

### 8. Settings (`/settings`)

| Screen | shadcn/ui Components | Notes |
|--------|---------------------|-------|
| Profile Settings | `Card`, `Input`, `Avatar`, `Button` | Edit name, bio, avatar |
| Privacy | `Switch`, `Card`, `Select` | Visibility controls |
| Connected Accounts | `Card`, `Button`, `Badge` | Fediverse connections |
| Notifications | `Switch`, `Card` | Push/email preferences |
| Appearance | `Select`, `Card` | Theme, units (km/mi) |

**Layout**: `Sidebar` with sections, form cards.

### 9. Global Navigation

| Element | shadcn/ui Components | Notes |
|---------|---------------------|-------|
| Top Nav | `Navigation Menu` or custom | Logo, search, notifications, profile |
| Bottom Nav (mobile) | Custom with `Button` (ghost) | Feed, Explore, Record, Profile |
| Sidebar (desktop) | `Sidebar` | Collapsible navigation |
| Search | `Command` | Global command palette (Cmd+K) |
| Breadcrumbs | `Breadcrumb` | Deep navigation |

---

## Component Priority (MVP)

### Phase 1 — Core (Week 1-2)
- `Card` — everything is a card
- `Button` — primary, secondary, ghost, destructive
- `Input`, `Label`, `Textarea` — forms
- `Badge` — sport types, status
- `Avatar` — user avatars
- `Separator` — visual dividers
- `Tabs` — profile, activity detail
- `Toast` — success/error feedback

### Phase 2 — Feed & Activity (Week 3-4)
- `Dialog` — create activity, confirmations
- `Sheet` — mobile drawers
- `Dropdown Menu` — activity actions
- `Tooltip` — stat explanations
- `Skeleton` — loading states
- `Empty` — empty states
- `Scroll Area` — long lists

### Phase 3 — Data & Charts (Week 5-6)
- `Table` / `DataTable` — splits, leaderboards
- `Chart` — speed/elevation charts
- `Calendar` — activity calendar
- `Progress` — goal progress
- `Slider` — filters

### Phase 4 — Advanced (Week 7+)
- `Command` — global search
- `Sidebar` — desktop navigation
- `Resizable` — dashboard panels
- `Carousel` — activity photos
- `Accordion` — FAQ, settings sections

---

## Layout Strategy (Impeccable Principles)

### Spacing
- Use Tailwind's 4px grid: `p-4`, `p-6`, `p-8`
- Consistent gaps: `gap-4` for cards, `gap-6` for sections
- Page padding: `px-4 py-6 md:px-6 lg:px-8`

### Typography (Impeccable)
- Headings: `font-bold tracking-tight`
- Body: `text-sm leading-relaxed`
- Muted: `text-muted-foreground`
- Scale: `text-xs`, `text-sm`, `text-base`, `text-lg`, `text-xl`, `text-2xl`, `text-3xl`

### Depth (Impeccable)
- Cards: `shadow-sm` (subtle) or `shadow-md` (elevated)
- Hover states: `hover:shadow-md transition-shadow`
- No heavy gradients — let Tailwind's shadow system handle depth
- Border: `border` for definition, not shadows everywhere

### Color Usage
- Primary (red): CTAs, active states, important actions
- Secondary (dark blue): Headings, strong text
- Muted: Supporting text, placeholders
- Sport colors: Badges, icons, chart lines only

---

## File Structure

```
src/
  components/
    ui/           # shadcn/ui components (auto-generated)
    layout/       # Layout wrappers (Header, Footer, Sidebar)
    activity/     # Activity-specific components
    feed/         # Feed components
    profile/      # Profile components
    charts/       # Chart components
  lib/
    utils.ts      # cn() helper, etc.
  styles/
    globals.css   # CSS variables, Tailwind config
```

---

## Next Steps

1. Initialize shadcn/ui in the project
2. Install core components (Phase 1)
3. Set up CSS variables with our color tokens
4. Build layout shell (Header, Sidebar, Main)
5. Build auth screens
6. Build feed with activity cards
