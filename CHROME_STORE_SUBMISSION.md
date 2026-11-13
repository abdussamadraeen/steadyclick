# Chrome Web Store Submission Kit: SteadyClick

This document contains all the copy, privacy declarations, and asset guidelines required to publish **SteadyClick** on the **Google Chrome Web Store** and **Microsoft Edge Add-ons**.

---

## 1. Store Listing Details

### Extension Name (SEO-Optimized)
> **SteadyClick - Web Accessibility & Tremor Mouse**

### Short Description (Max 132 chars)
> Assists people with hand tremors, low vision, and sensory overload. Adaptive mouse smoothing, magnetic clicks, and text clarity.

### Category
> **Accessibility** (Secondary: Productivity)

### Language
> **English (United States)**

---

## 2. Detailed Store Description (Copy & Paste)

```markdown
SteadyClick is an all-in-one assistive companion designed to make the web accessible, friendly, and frustration-free for everyone.

Whether you experience hand tremors (Parkinson’s, essential tremor, arthritis), low vision, or sensory overload, SteadyClick runs silently in the background, adapting every website to fit your physical needs.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✨ KEY ASSISTIVE FEATURES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🛡️ STEADY MOUSE (Tremor Suppression)
• Automatically dampens involuntary hand shakes and motor tremors using an adaptive velocity algorithm.
• When moving slowly to aim or click, the cursor locks rock-steady. When moving fast across the screen, it responds with zero input lag.
• 3 customizable sensitivity modes: Gentle, Balanced, and Strong.

🧲 MAGNETIC CLICK (Fitts' Law Target Capture)
• Never miss a tiny button again.
• As your cursor approaches a link, input field, or button, a dynamic magnetic bubble locks onto the target.
• Even if your finger shakes and clicks slightly outside the target, SteadyClick redirects the click straight into the button.

👁️ HIGH CLARITY VISION (WCAG 3.0 APCA Contrast)
• Sharpens washed-out, low-contrast text across the web.
• Evaluates text against modern human perceptual contrast algorithms to ensure maximum readability without distorting photos.

🧘 CALM MODE (Sensory Overload Protection)
• Designed for neurodivergent users, ADHD, and migraine sufferers.
• Instantly freezes aggressive animations, flashing banners, and auto-playing videos across all open tabs with a single switch.

📖 SIMPLE WORDS (Jargon Explainer)
• Identifies complex, multisyllabic medical, legal, and financial jargon and provides instant plain-English clarity.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔒 AIR-GAPPED PRIVACY PROMISE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
• 100% Offline: Operates entirely client-side using a fast, self-contained WebAssembly engine.
• Zero Data Collection: SteadyClick contains `connect-src: 'none'`. The browser physically prohibits the extension from sending even a single byte of your browsing data over the internet.
• No accounts, no sign-ups, and no tracking.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
♿ WHO IS THIS FOR?
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
• Individuals with Parkinson's, Essential Tremor, Multiple Sclerosis (MS), Cerebral Palsy, or Arthritis.
• Elderly web users struggling with tiny clickable targets.
• Users with low vision or eye strain.
• Individuals with ADHD, autism, or sensory processing sensitivity.
```

---

## 3. Official Single-Purpose & Privacy Policy Declaration

When submitting on the Chrome Developer Dashboard, fill in the **Privacy Tab** with these exact answers:

1. **Single Purpose Description:**
   > "SteadyClick provides client-side motor, visual, and cognitive accessibility enhancements—including tremor dampening, magnetic click targeting, and high contrast—to make web navigation accessible for individuals with disabilities."

2. **Permission Justification:**
   * **`storage`**: Used exclusively to save the user's local accessibility settings (e.g. tremor filter strength and active toggles) on their own device.

3. **Data Usage Certification:**
   * Does this extension collect or transmit personal user data? **NO.**
   * Does it use remote code? **NO.** (All logic is bundled locally in `engine_core.wasm`).

---

## 4. How to Package the Extension for Submission

To create the `.zip` file for the Chrome Web Store:
1. Select all files inside the `extension/` folder:
   * `manifest.json`
   * `icons/`
   * `src/`
   * `wasm/`
2. Compress them into a single zip file: `steadyclick-v1.0.0.zip`.
3. Upload `steadyclick-v1.0.0.zip` to the [Chrome Developer Dashboard](https://chrome.google.com/webstore/devconsole).
