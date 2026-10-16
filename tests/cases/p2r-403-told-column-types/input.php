<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// Mago knows each column's type, so a read goes where an int, a string or a bool must go.
final class Item extends Model
{
    protected $casts = [
        'active' => 'boolean',
    ];
}

function nameLength(Item $item): int
{
    return strlen($item->name);
}

function doubled(Item $item): int
{
    return $item->stock * 2 + $item->id;
}

function bar(Item $item): string
{
    if ($item->active) {
        return substr($item->name, 0, $item->stock);
    }
    return strtoupper($item->name);
}
