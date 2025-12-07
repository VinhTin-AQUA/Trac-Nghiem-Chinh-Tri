import { Routes } from '@angular/router';
import { MainLayout } from './shared/layouts/main-layout/main-layout';
import { MainRoutes } from './core/constants/routes-consts';
import { Home } from './pages/home/home';
import { StartExam } from './pages/start-exam/start-exam';
import { QuestionBank } from './pages/question-bank/question-bank';

export const routes: Routes = [
    {
        path: '',
        component: MainLayout,
        children: [
            {
                path: MainRoutes.HOME,
                component: Home,
            },
            {
                path: MainRoutes.STARTEXAM,
                component: StartExam,
            },
            {
                path: MainRoutes.QUESTIONBANK,
                component: QuestionBank,
            },
            { path: '', redirectTo: MainRoutes.HOME, pathMatch: 'full' },
        ],
    },
    { path: '**', redirectTo: '' },
];
