<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// A cast gives a column its type, whatever the migration declared; casts() wins over
// $casts on a shared key.
final class Flag extends Model
{
    protected $casts = [
        'enabled' => 'boolean',
        'visible' => 'bool',
        'counted' => 'string',
        'weight' => 'string',
        'ratio' => 'decimal:2',
    ];

    protected function casts(): array
    {
        return [
            'counted' => 'integer',
            'opened_at' => 'string',
            'code' => 'int',
        ];
    }
}

function summary(Flag $flag): string
{
    return $flag->weight . '/' . $flag->opened_at . '/' . ($flag->counted + $flag->code);
}

function shown(Flag $flag): bool
{
    return $flag->enabled;
}

function maybeShown(Flag $flag): ?bool
{
    return $flag->visible;
}
