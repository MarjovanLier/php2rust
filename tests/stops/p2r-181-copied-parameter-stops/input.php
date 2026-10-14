<?php

declare(strict_types=1);

final class Counter
{
    public int $n = 0;

    public function add(): void
    {
        $this->n += 1;
    }
}

final class Box
{
    public function __construct(public string $label)
    {
        $label = 'x';
    }
}

/** @param list<string> $xs */
function names(array $xs, string $x): int
{
    echo $x;
    foreach ($xs as $x) {
        echo $x;
    }
    return 0;
}

function wide(?string $customerBillingAddressOverride, bool $c): ?string
{
    if ($c) {
        $customerBillingAddressOverride = null;
    }
    return $customerBillingAddressOverride;
}

final class Text
{
    public static function shout(string $customerBillingAddressOverrideText): string
    {
        $customerBillingAddressOverrideText .= '!';
        return $customerBillingAddressOverrideText;
    }
}

/** @param list<Counter> $cs */
function grow(array $cs): int
{
    $cs[] = new Counter();
    return count($cs);
}

function bump(Counter $c): void
{
    $c->add();
    $c = new Counter();
    $c->add();
}
