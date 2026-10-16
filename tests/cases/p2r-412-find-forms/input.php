<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class City extends Model
{
    protected $fillable = ['name'];

    // The table has no timestamps, which Laravel would write on create.
    public $timestamps = false;
}

function cityName(?City $city): string
{
    return $city === null ? 'none' : $city->name;
}

// find gives the row with that id, or null; the ids count from 1 in creation order.
echo cityName(City::find(1)), "\n";
City::create(['name' => 'Veghel']);
City::query()->create(['name' => 'Uden']);
$id = 2;
echo cityName(City::find(1)), ' ', cityName(City::query()->find($id)), ' ', cityName(City::find($id + 1)), "\n";
