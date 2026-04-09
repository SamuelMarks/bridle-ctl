import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component } from '@angular/core';
import { OrgListComponent } from './org-list.component';
import { By } from '@angular/platform-browser';

@Component({
  template: `<app-org-list [orgs]="orgs" (select)="onSelect($event)"></app-org-list>`,
  imports: [OrgListComponent]
})
class TestHostComponent {
  orgs = [
    { id: '1', name: 'org-1', provider: 'github' }
  ];
  selectedId = '';

  onSelect(id: string) {
    this.selectedId = id;
  }
}

describe('OrgListComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, OrgListComponent]
    }).compileComponents();

    fixture = TestBed.createComponent(TestHostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should render table with orgs', () => {
    const trs = fixture.debugElement.queryAll(By.css('tbody tr'));
    expect(trs.length).toBe(1);
    expect(trs[0].nativeElement.textContent).toContain('org-1');
    expect(trs[0].nativeElement.textContent).toContain('github');
  });

  it('should emit select event on name click', () => {
    const link = fixture.debugElement.query(By.css('.text-bold')).nativeElement;
    link.click();
    expect(component.selectedId).toBe('1');
  });

  it('should emit select event on action click', () => {
    const action = fixture.debugElement.query(By.css('.text-muted')).nativeElement;
    action.click();
    expect(component.selectedId).toBe('1');
  });
});
