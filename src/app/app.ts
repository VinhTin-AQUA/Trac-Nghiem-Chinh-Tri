import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { LanguageService } from './core/services/language-service';
import { Loader } from './shared/components/loader/loader';
import { DialogService } from './core/services/dialog-service';

@Component({
    selector: 'app-root',
    imports: [RouterOutlet, Loader],
    templateUrl: './app.html',
    styleUrl: './app.css',
})
export class App {
    protected readonly title = signal('Trac-Nghiem-Chinh-Tri');

    constructor(private languageService: LanguageService, public dialogService: DialogService) {}
}
