<?php

declare(strict_types=1);

namespace Catalog;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;

// Items with a boolean cast and when() filters, written by hand for the harness.
final class Item extends Model
{
    protected $fillable = ['name', 'qty', 'active', 'colour'];

    protected $casts = ['active' => 'boolean'];

    public $timestamps = false;
}

function stock(string $name, int $qty, bool $active, ?string $colour): int
{
    $item = Item::create(['name' => $name, 'qty' => $qty, 'active' => $active, 'colour' => $colour]);
    return $item->id;
}

function isActive(int $id): bool
{
    $item = Item::findOrFail($id);
    return $item->active;
}

function colourOf(int $id): ?string
{
    $item = Item::find($id);
    if ($item === null) {
        return null;
    }
    return $item->colour;
}

/**
 * @return Collection<int, Item>
 */
function filtered(?string $name, ?int $qty, bool $activeOnly, ?string $colour): Collection
{
    return Item::query()
        ->when($name, fn (Builder $query, string $name) => $query->where('name', $name))
        ->when($qty, fn (Builder $query, int $qty) => $query->where('qty', $qty))
        ->when($activeOnly, fn (Builder $query) => $query->where('active', true))
        ->when($colour, static function (Builder $query, string $colour): void {
            $query->where('colour', $colour);
        })
        ->get();
}

/**
 * @return list<string>
 */
function names(?string $name, ?int $qty, bool $activeOnly, ?string $colour): array
{
    $found = [];
    foreach (filtered($name, $qty, $activeOnly, $colour) as $item) {
        $found[] = $item->name;
    }
    return $found;
}

/**
 * @return list<int>
 */
function ids(?string $name, ?int $qty, bool $activeOnly, ?string $colour): array
{
    $found = [];
    foreach (filtered($name, $qty, $activeOnly, $colour) as $item) {
        $found[] = $item->id;
    }
    return $found;
}

function matches(?string $name, ?int $qty, bool $activeOnly, ?string $colour): int
{
    return filtered($name, $qty, $activeOnly, $colour)->count();
}

function noneMatch(?string $name, ?int $qty, bool $activeOnly, ?string $colour): bool
{
    return filtered($name, $qty, $activeOnly, $colour)->isEmpty();
}

function stockedName(string $name, int $qty): string
{
    $id = stock($name, $qty, true, null);
    $item = Item::findOrFail($id);
    return $item->name . ' x' . $item->qty;
}
