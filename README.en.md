# Vietnamese Book Publishing Registration RSS Feed Tracker

This project generates an RSS feed to track the official book publishing registrations in Vietnam. It provides updates every 7 days about newly registered books (if there any).

## How to Use

You can use any RSS feed reader you like (I think). Personally, I use Thunderbird. To get the RSS link, go to /feed/rss/{title}, choose the feed you want to follow, click on "raw," copy the link, and paste it into your RSS Feed Tracker.

Example valid url: https://raw.githubusercontent.com/Irilith/VBT/refs/heads/main/feed/rss/Majo_no_Tabitabi.rss

## Self-Hosting

If you wish to self-host this project, the recommended way is to fork this repository and enable GitHub Actions in your fork.

## Setup Secrets

Remember to set up the webhook secrets for `"WEEKLY"` and `"PROCESSED"`. If you don't, you will encounter errors (I'll fix it later, but I'm lazy for now).

## Suggest a Book to Watchlist

If you'd like to suggest a book for tracking, you can do so by opening an issue or pull request if you know how to do it. Alternatively, you can also suggest books directly in my Discord server. The book must be well-known (based on the votes of the issue or pull request, or based on my own knowledge) or something that I want to track.

## Join the Discord Server

If you'd like to chat, feel free to join my Discord server: [discord.gg](https://discord.gg/VJ57nka8G6)
