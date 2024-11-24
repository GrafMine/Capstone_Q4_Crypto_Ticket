'use client'

import { AppHero } from '../ui/ui-layout'

const FeatureCard = ({ title, description }) => (
    <div className="bg-white/5 hover:bg-white/10 transition-all">
        <div className="p-6 flex flex-col items-center text-center space-y-4">
            <h3 className="font-semibold text-lg">{title}</h3>
            <p className="text-muted-foreground">{description}</p>
        </div>
    </div>
)

const UseCaseCard = ({ title, points }) => (
    <div className="bg-white/5 p-6 rounded-lg">
        <h3 className="text-lg font-semibold mb-4">{title}</h3>
        <ul className="space-y-2">
            {points.map((point, index) => (
                <li key={index} className="flex items-start">
                    <span className="text-primary mr-2">•</span>
                    <span className="text-muted-foreground">{point}</span>
                </li>
            ))}
        </ul>
    </div>
)

export default function DashboardFeature() {
        const features = [
            {
                title: "Dynamic Numbers",
                description: "Your ticket numbers change automatically with each new purchase, giving you fresh winning opportunities."
            },
            {
                title: "Passive Income",
                description: "Earn a portion of every ticket purchase made after yours, creating a steady stream of passive income."
            },
            {
                title: "Marketplace",
                description: "Create and sell your own tickets, maximizing profit potential in the ticket marketplace."
            },
            {
                title: "Token Distribution",
                description: "Perfect for airdrops and fair token distribution while maintaining market liquidity."
            }
        ]
        
        const useCases = [
            {
                title: "Investment Strategy",
                points: [
                    "Buy once and collect passive profit until lottery completion",
                    "Automatic number changes increase winning chances",
                    "Earn from every subsequent ticket purchase",
                    "Long-term return potential exceeds initial investment"
                ]
            },
            {
                title: "Token Distribution Platform",
                points: [
                    "Distribute tokens through lottery participation",
                    "Generate demand through gamified distribution",
                    "Maintain market liquidity with paid participation",
                    "Create engaging token distribution mechanics"
                ]
            }
        ]
        
return (
    <div>
        <AppHero
            title="Crypto Ticket"
            subtitle="is a long-term electronic lottery where your numbers change with each new ticket sale, giving you a new chance to win and passive income."
        />
        <div className="max-w-7xl mx-auto py-12 px-4">
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-16">
                {features.map((feature, index) => (
                    <FeatureCard key={index} {...feature} />
                ))}
            </div>
            <h2 className="text-2xl font-bold text-center mb-8">Use Cases</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                {useCases.map((useCase, index) => (
                    <UseCaseCard key={index} {...useCase} />
                ))}
            </div>
        </div>
    </div>
  )
}
