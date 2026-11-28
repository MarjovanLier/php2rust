<?php

declare(strict_types=1);

// The app's one config value, which its middleware sends as a header.
return [
    'name' => env('DEMO_NAME', 'demo'),
];
