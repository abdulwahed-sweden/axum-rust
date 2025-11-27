pub fn header() -> String {
    r#"<header class="fixed top-0 left-0 right-0 z-50 border-b border-white/5 bg-[#0a0a0f]/80 backdrop-blur-xl">
    <div class="max-w-6xl mx-auto px-6 h-14 flex items-center justify-between">
      <a href="/" class="flex items-center gap-2.5 font-semibold text-white/90 hover:text-white transition-colors">
        <svg class="w-5 h-5 text-violet-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
        </svg>
        <span class="font-display">Axum API</span>
      </a>
      <nav class="flex items-center gap-1">
        <a href="/users" class="px-3 py-1.5 text-sm text-white/50 hover:text-white/90 hover:bg-white/5 rounded-lg transition-all">Users</a>
        <a href="/products" class="px-3 py-1.5 text-sm text-white/50 hover:text-white/90 hover:bg-white/5 rounded-lg transition-all">Products</a>
        <a href="/orders" class="px-3 py-1.5 text-sm text-white/50 hover:text-white/90 hover:bg-white/5 rounded-lg transition-all">Orders</a>
        <div class="w-px h-4 bg-white/10 mx-2"></div>
        <a href="https://github.com" target="_blank" class="p-2 text-white/40 hover:text-white/90 hover:bg-white/5 rounded-lg transition-all">
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
          </svg>
        </a>
      </nav>
    </div>
  </header>"#.to_string()
}

pub fn footer() -> String {
    r#"<footer class="border-t border-white/5 bg-[#0a0a0f]/50">
    <div class="max-w-6xl mx-auto px-6 py-8 flex items-center justify-between text-sm">
      <p class="text-white/30">&copy; 2024 Axum API. Built with Rust.</p>
      <div class="flex items-center gap-6 text-white/30">
        <a href="https://docs.rs/axum" target="_blank" class="hover:text-white/60 transition-colors">Documentation</a>
        <a href="https://github.com" target="_blank" class="hover:text-white/60 transition-colors">GitHub</a>
      </div>
    </div>
  </footer>"#.to_string()
}

pub fn landing_page() -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Axum API</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Space+Grotesk:wght@500;600;700&display=swap" rel="stylesheet">
  <script src="https://cdn.tailwindcss.com"></script>
  <script>
    tailwind.config = {{
      theme: {{
        extend: {{
          fontFamily: {{
            sans: ['Inter', 'system-ui', 'sans-serif'],
            display: ['Space Grotesk', 'system-ui', 'sans-serif'],
          }},
        }},
      }},
    }}
  </script>
  <style>
    .grid-bg {{
      background-image:
        linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px),
        linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px);
      background-size: 60px 60px;
      animation: grid-move 20s linear infinite;
    }}
    @keyframes grid-move {{
      0% {{ background-position: 0 0; }}
      100% {{ background-position: 60px 60px; }}
    }}
    .gradient-text {{
      background: linear-gradient(135deg, #a78bfa 0%, #06b6d4 50%, #a78bfa 100%);
      background-size: 200% 200%;
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
      animation: gradient-shift 8s ease infinite;
    }}
    @keyframes gradient-shift {{
      0%, 100% {{ background-position: 0% 50%; }}
      50% {{ background-position: 100% 50%; }}
    }}
    .card {{
      background: rgba(255,255,255,0.02);
      border: 1px solid rgba(255,255,255,0.06);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    }}
    .card:hover {{
      background: rgba(255,255,255,0.04);
      border-color: transparent;
      transform: scale(1.02);
      box-shadow:
        0 0 0 1px rgba(167, 139, 250, 0.3),
        0 20px 40px -20px rgba(0,0,0,0.5);
    }}
    .card:hover .card-glow {{
      opacity: 1;
    }}
    .card-glow {{
      position: absolute;
      inset: -1px;
      background: linear-gradient(135deg, rgba(167,139,250,0.2), rgba(6,182,212,0.2));
      border-radius: inherit;
      opacity: 0;
      transition: opacity 0.3s ease;
      z-index: -1;
      filter: blur(8px);
    }}
  </style>
</head>
<body class="bg-[#0a0a0f] text-white/90 min-h-screen font-sans antialiased">
  <div class="fixed inset-0 grid-bg pointer-events-none"></div>
  <div class="fixed inset-0 bg-gradient-to-b from-violet-500/5 via-transparent to-cyan-500/5 pointer-events-none"></div>

  {header}

  <main class="relative pt-14">
    <!-- Hero -->
    <section class="max-w-6xl mx-auto px-6 pt-24 pb-16 text-center">
      <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white/5 border border-white/10 text-white/60 text-xs font-medium mb-8">
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
        v1.0 — Production Ready
      </div>
      <h1 class="font-display text-5xl md:text-6xl lg:text-7xl font-bold tracking-tight mb-6">
        <span class="gradient-text">Axum API</span>
      </h1>
      <p class="text-white/40 text-lg md:text-xl max-w-2xl mx-auto leading-relaxed">
        A blazing-fast REST API built with Rust. Type-safe, async-first, and ready for production.
      </p>
    </section>

    <!-- Stats -->
    <section class="max-w-6xl mx-auto px-6 pb-16">
      <div class="flex items-center justify-center gap-8 text-sm text-white/30">
        <div class="flex items-center gap-2">
          <svg class="w-4 h-4 text-violet-400/60" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path><circle cx="12" cy="7" r="4"></circle></svg>
          <span>5 Users</span>
        </div>
        <div class="w-1 h-1 rounded-full bg-white/20"></div>
        <div class="flex items-center gap-2">
          <svg class="w-4 h-4 text-cyan-400/60" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path></svg>
          <span>8 Products</span>
        </div>
        <div class="w-1 h-1 rounded-full bg-white/20"></div>
        <div class="flex items-center gap-2">
          <svg class="w-4 h-4 text-emerald-400/60" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="1" y="4" width="22" height="16" rx="2" ry="2"></rect><line x1="1" y1="10" x2="23" y2="10"></line></svg>
          <span>5 Orders</span>
        </div>
      </div>
    </section>

    <!-- Endpoints -->
    <section class="max-w-5xl mx-auto px-6 pb-24">
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">

        <!-- GET /users -->
        <article class="card relative rounded-2xl p-5 backdrop-blur-sm cursor-pointer">
          <div class="card-glow rounded-2xl"></div>
          <div class="flex items-center gap-3 mb-3">
            <span class="px-2 py-0.5 rounded-md bg-emerald-500/20 text-emerald-400 text-xs font-semibold uppercase tracking-wide">GET</span>
            <code class="text-white/80 text-sm font-mono">/users</code>
          </div>
          <p class="text-white/40 text-sm leading-relaxed">List all users in the system</p>
        </article>

        <!-- GET /users/{{id}} -->
        <article class="card relative rounded-2xl p-5 backdrop-blur-sm cursor-pointer">
          <div class="card-glow rounded-2xl"></div>
          <div class="flex items-center gap-3 mb-3">
            <span class="px-2 py-0.5 rounded-md bg-emerald-500/20 text-emerald-400 text-xs font-semibold uppercase tracking-wide">GET</span>
            <code class="text-white/80 text-sm font-mono">/users/{{id}}</code>
          </div>
          <p class="text-white/40 text-sm leading-relaxed">Fetch a user by their ID</p>
        </article>

        <!-- POST /users -->
        <article class="card relative rounded-2xl p-5 backdrop-blur-sm cursor-pointer">
          <div class="card-glow rounded-2xl"></div>
          <div class="flex items-center gap-3 mb-3">
            <span class="px-2 py-0.5 rounded-md bg-blue-500/20 text-blue-400 text-xs font-semibold uppercase tracking-wide">POST</span>
            <code class="text-white/80 text-sm font-mono">/users</code>
          </div>
          <p class="text-white/40 text-sm leading-relaxed">Create a new user account</p>
        </article>

        <!-- GET /products -->
        <article class="card relative rounded-2xl p-5 backdrop-blur-sm cursor-pointer">
          <div class="card-glow rounded-2xl"></div>
          <div class="flex items-center gap-3 mb-3">
            <span class="px-2 py-0.5 rounded-md bg-emerald-500/20 text-emerald-400 text-xs font-semibold uppercase tracking-wide">GET</span>
            <code class="text-white/80 text-sm font-mono">/products</code>
          </div>
          <p class="text-white/40 text-sm leading-relaxed">Browse the product catalog</p>
        </article>

        <!-- GET /orders -->
        <article class="card relative rounded-2xl p-5 backdrop-blur-sm cursor-pointer">
          <div class="card-glow rounded-2xl"></div>
          <div class="flex items-center gap-3 mb-3">
            <span class="px-2 py-0.5 rounded-md bg-emerald-500/20 text-emerald-400 text-xs font-semibold uppercase tracking-wide">GET</span>
            <code class="text-white/80 text-sm font-mono">/orders</code>
          </div>
          <p class="text-white/40 text-sm leading-relaxed">View all order records</p>
        </article>

        <!-- POST /orders -->
        <article class="card relative rounded-2xl p-5 backdrop-blur-sm cursor-pointer">
          <div class="card-glow rounded-2xl"></div>
          <div class="flex items-center gap-3 mb-3">
            <span class="px-2 py-0.5 rounded-md bg-blue-500/20 text-blue-400 text-xs font-semibold uppercase tracking-wide">POST</span>
            <code class="text-white/80 text-sm font-mono">/orders</code>
          </div>
          <p class="text-white/40 text-sm leading-relaxed">Place a new order</p>
        </article>

      </div>
    </section>
  </main>

  {footer}
</body>
</html>"##,
        header = header(),
        footer = footer()
    )
}
