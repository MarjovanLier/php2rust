<?php

declare(strict_types=1);

namespace Pricing;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\Optional;

// laravel-data classes with Optional properties and a backed enum, written by hand for the
// harness; no database.
enum Currency: string
{
    case Euro = 'eur';
    case Dollar = 'usd';
    case Rand = 'zar';
}

final class Price extends Data
{
    public function __construct(
        public int $cents,
        public Currency $currency,
        public string|Optional $label,
        public int|Optional $discount,
    ) {}
}

function labelOf(?string $label): string|Optional
{
    if ($label === null) {
        return Optional::create();
    }
    return $label;
}

function discountOf(int $discount): int|Optional
{
    if ($discount > 0) {
        return $discount;
    }
    return Optional::create();
}

function priced(int $cents, string $code, ?string $label, int $discount): Price
{
    return new Price($cents, Currency::from($code), labelOf($label), discountOf($discount));
}

function symbol(string $code): string
{
    $currency = Currency::tryFrom($code);
    if ($currency === null) {
        return '?';
    }
    return match ($currency) {
        Currency::Euro => 'EUR',
        Currency::Dollar => 'USD',
        Currency::Rand => 'R',
    };
}

function caseName(string $code): string
{
    $currency = Currency::tryFrom($code);
    if ($currency === null) {
        return 'unknown';
    }
    return $currency->name;
}

function tag(int $cents, string $code, ?string $label, int $discount): string
{
    $price = priced($cents, $code, $label, $discount);
    $named = $price->label instanceof Optional ? 'unlabelled' : 'labelled';
    $off = $price->discount instanceof Optional ? 'full price' : 'discounted';
    return $price->cents . ' ' . $price->currency->value . ', ' . $named . ', ' . $off;
}

function isDiscounted(int $discount): bool
{
    return !priced(100, 'eur', null, $discount)->discount instanceof Optional;
}

function isLabelled(?string $label): bool
{
    return !priced(100, 'eur', $label, 0)->label instanceof Optional;
}

function band(int $cents): string
{
    if ($cents <= 0) {
        return 'free';
    }
    if ($cents < 1000) {
        return 'cheap';
    }
    if ($cents < 10000) {
        return 'mid';
    }
    return 'dear';
}
