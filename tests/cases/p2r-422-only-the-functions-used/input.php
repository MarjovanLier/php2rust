<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\ModelNotFoundException;

// Each store holds only the functions its model's code calls: findOrFail alone for Place,
// create and firstOrFail for Item.
final class Place extends Model
{
    public $timestamps = false;
}

final class Item extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;
}

function placeName(int $id): string
{
    try {
        $place = Place::findOrFail($id);
        return $place->name;
    } catch (ModelNotFoundException $e) {
        return $e->getMessage();
    }
}

function itemNamed(string $name): int
{
    $item = Item::query()->where('name', $name)->firstOrFail();
    return $item->id;
}

Item::create(['name' => 'one']);
Item::create(['name' => 'two']);
echo placeName(1), "\n";
echo itemNamed('two'), "\n";
