<?php

declare(strict_types=1);

readonly class Leaf
{
    public function __construct(public string $textValueWithAVeryLongName) {}
}

readonly class Branch
{
    public function __construct(public Leaf $leafHolderWithLongName) {}

    public function size(): int
    {
        return strlen($this->leafHolderWithLongName->textValueWithAVeryLongName);
    }
}

final class Defaults
{
    public string $note = 'a very long default string that is wider than sixty columns';
}

final readonly class Limits
{
    const WIDE = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';

    public function __construct(public int $n) {}
}

final class Text
{
    public static function shout(string $customerBillingAddressOverrideText): string
    {
        $customerBillingAddressOverrideText .= '!';
        return $customerBillingAddressOverrideText;
    }
}

function kind(int $n): string
{
    return match ($n) {
        1 => $n > 0 ? 'positive and one' : 'never',
        default => 'other',
    };
}

function wide(?string $customerBillingAddressOverride, bool $c): ?string
{
    if ($c) {
        $customerBillingAddressOverride = null;
    }
    return $customerBillingAddressOverride;
}

function checked(int $numberOfItemsInTheBasket, ?int $discountPercentageForTheCustomer): int
{
    if ($numberOfItemsInTheBasket > 0 && $discountPercentageForTheCustomer !== null && $numberOfItemsInTheBasket < 1000) {
        return $numberOfItemsInTheBasket * $discountPercentageForTheCustomer;
    }
    return 0;
}

function picked(bool $on): string
{
    return $on ? 'abcdefghijklmnopqrstuvwxyz and more letters' : 'nopqrstuvwxyz and fewer';
}

$branch = new Branch(new Leaf('a leaf with some text in it that is long enough to matter'));
echo $branch->size(), ' ', (new Defaults())->note, "\n";
echo strlen(Limits::WIDE), ' ', Text::shout('hello'), ' ', kind(1), ' ', kind(2), "\n";
echo wide('kept', false) ?? 'cleared', ' ', wide('kept', true) ?? 'cleared', "\n";
echo checked(3, 10), ' ', checked(3, null), ' ', picked(true), ' ', picked(false), "\n";
echo strlen('This string literal is passed to a call and it is far too long for one line of Rust'), "\n";
