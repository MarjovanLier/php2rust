<?php

declare(strict_types=1);

namespace App\Enums;

use App\Contracts\Labelable;

enum Kind: string implements Labelable
{
    case Quote = 'quote';
    case Order = 'order';

    public function label(): string
    {
        return match ($this) {
            self::Quote => 'Quote',
            self::Order => 'Order',
        };
    }
}
